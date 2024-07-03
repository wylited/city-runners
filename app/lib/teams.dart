import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'globals.dart' as globals;

// Define TeamType Enum
enum TeamType { Seeker, Hider, Spectator }

extension ParseToString on TeamType {
  String toShortString() {
    return this.toString().split('.').last;
  }
}

// Define Team Model
class Team {
  final String name;
  final List<String> players;
  final TeamType ttype;

  Team({required this.name, required this.players, required this.ttype});

  factory Team.fromJson(Map<String, dynamic> json) {
    return Team(
      name: json['name'],
      players: List<String>.from(json['players']),
      ttype: TeamType.values
          .firstWhere((e) => e.toString() == 'TeamType.${json['ttype']}'),
    );
  }

  Map<String, dynamic> toJson() => {
        'name': name,
        'players': players,
        'ttype': ttype.toShortString(),
      };
}

class TeamListPage extends StatefulWidget {
  @override
  _TeamListPageState createState() => _TeamListPageState();
}

class _TeamListPageState extends State<TeamListPage> {
  List<Team> _teams = [];
  bool _isReady = false;

  @override
  void initState() {
    super.initState();
    _fetchTeams();
  }

  Future<void> _fetchTeams() async {
    String jsonData = "";
    final prefs = await SharedPreferences.getInstance();
    String token = prefs.getString('jwt') ?? "no token";

    final headers = {
      'Authorization': 'Bearer $token',
    };

    final response = await http.get(
      Uri.parse('https://${globals.server_address}/teams'),
      headers: headers,
    );

    if (response.statusCode == 200) {
      jsonData = response.body;
    } else {
      print('failed at data fetching');
    }

    if (jsonData == "") {
      jsonData = '''
    [
      {
        "name": "example team",
        "players": ["pulling teams failed"],
        "ttype": "Spectator"
      }
    ]
    ''';
    }
    List<dynamic> jsonList = jsonDecode(jsonData);
    setState(() {
      _teams = jsonList.map((json) => Team.fromJson(json)).toList();
    });
  }

  Future<void> _refreshTeams() async {
    await _fetchTeams();
  }

  Future<void> _addNewTeam(String name) async {
    // Add new team to the server
    final prefs = await SharedPreferences.getInstance();
    String token = prefs.getString('jwt') ?? "no token";

    final headers = {
      'Authorization': 'Bearer $token',
    };

    try {
      final response = await http.post(
        Uri.parse('https://${globals.server_address}/teams/$name'),
        headers: headers,
      );

      if (response.statusCode == 200) {
        print('success');
      } else {
        print('failed at creation');
      }
    } catch (e) {
      print('Error: $e');
    }

    await _refreshTeams();
  }

  Future<void> _joinTeam(String teamName) async {
    // Join team on the server
    final prefs = await SharedPreferences.getInstance();
    String token = prefs.getString;
    final headers = {
      'Authorization': 'Bearer $token',
    };

    final response = await http.post(
      Uri.parse('https://${globals.server_address}/teams/$teamName/join'),
      headers: headers,
    );
    await _refreshTeams();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Teams'),
      ),
      body: RefreshIndicator(
        onRefresh: _refreshTeams,
        child: ListView.builder(
          itemCount: _teams.length,
          itemBuilder: (context, index) {
            final team = _teams[index];
            return ListTile(
              title: Text(team.name),
              subtitle: Text(
                  'Players: ${team.players.join(', ')} | Type: ${team.ttype.toShortString()}'),
              trailing: IconButton(
                icon: Icon(Icons.add),
                onPressed: () {
                  _joinTeam(team.name);
                },
              ),
            );
          },
        ),
      ),
      bottomNavigationBar: Padding(
        padding: EdgeInsets.all(8.0),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                onPressed: () => _showAddTeamDialog(context),
                icon: Icon(Icons.add),
                label: Text('Create New Team'),
              ),
            ),
            SizedBox(height: 8),
            SizedBox(
              width: double.infinity,
              child: ElevatedButton.icon(
                style: ButtonStyle(
                  backgroundColor: MaterialStateProperty.all(
                      _isReady ? Colors.green : Colors.grey),
                ),
                onPressed: () {
                  setState(() {
                    _isReady = !_isReady;
                  });
                },
                icon: Icon(Icons.check),
                label: Text('Ready'),
              ),
            ),
          ],
        ),
      ),
    );
  }

  void _showAddTeamDialog(BuildContext context) {
    TextEditingController teamNameController = TextEditingController();

    showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text('New Team'),
          content: TextField(
            controller: teamNameController,
            decoration: InputDecoration(hintText: 'Enter team name'),
          ),
          actions: [
            TextButton(
              onPressed: () {
                Navigator.of(context).pop();
              },
              child: Text('Cancel'),
            ),
            TextButton(
              onPressed: () {
                if (teamNameController.text.isNotEmpty) {
                  _addNewTeam(teamNameController.text);
                }
                Navigator.of(context).pop();
              },
              child: Text('Add'),
            ),
          ],
        );
      },
    );
  }
}
