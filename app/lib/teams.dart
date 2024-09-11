import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'main.dart';
import 'globals.dart' as globals;
import 'dart:async';

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
  String? _currentTeam;
  bool _isReady = false;

  @override
  void initState() {
    super.initState();
    _fetchTeams();
    _getCurrentTeam();
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
    // check if any of the teams in the json list contains our username under players
    // if so set that team to our current team
    String? currentTeam;
    for (var teamJson in jsonList) {
      Team team = Team.fromJson(teamJson);
      for (String player in team.players) {
        // the player is written as username (readystate), so check accordingly
        if (player.split(' ')[0] == globals.username) {
          currentTeam = team.name;
          break;
        }
      }
    }

    @override
    void dispose() {
      super.dispose();
    }

    setState(() {
      _teams = jsonList.map((json) => Team.fromJson(json)).toList();
      _currentTeam = currentTeam;
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
        setState(() {
          _currentTeam = name;
        });
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
    if (_currentTeam != null) {
      // Prompt user to leave current team first
      _showLeaveCurrentTeamDialog();
      return;
    }

    // Join team on the server
    final prefs = await SharedPreferences.getInstance();
    String token = prefs.getString('jwt') ?? "no token";
    final headers = {
      'Authorization': 'Bearer $token',
    };

    final response = await http.post(
      Uri.parse('https://${globals.server_address}/teams/$teamName/join'),
      headers: headers,
    );

    if (response.statusCode == 200) {
      setState(() {
        _currentTeam = teamName;
      });
    } else {
      print('Failed to join team');
    }

    await _refreshTeams();
  }

  Future<void> _leaveTeam() async {
    if (_currentTeam == null) {
      return;
    }

    // Leave current team on the server
    final prefs = await SharedPreferences.getInstance();
    String token = prefs.getString('jwt') ?? "no token";
    final headers = {
      'Authorization': 'Bearer $token',
    };

    final response = await http.post(
      Uri.parse('https://${globals.server_address}/teams/$_currentTeam/leave'),
      headers: headers,
    );

    if (response.statusCode == 200) {
      setState(() {
        _currentTeam = null;
      });
    } else {
      print('Failed to leave team');
    }

    await _refreshTeams();
  }

  Future<void> _getCurrentTeam() async {
    final prefs = await SharedPreferences.getInstance();
    setState(() {
      _currentTeam = prefs.getString('currentTeam');
    });
  }

  void _showLeaveCurrentTeamDialog() {
    showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text('Leave Current Team'),
          content: Text(
              'You need to leave your current team before joining a new one. Do you want to leave your current team?'),
          actions: [
            TextButton(
              onPressed: () {
                Navigator.of(context).pop();
              },
              child: Text('Cancel'),
            ),
            TextButton(
              onPressed: () {
                _leaveTeam();
                Navigator.of(context).pop();
              },
              child: Text('Leave'),
            ),
          ],
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Teams'),
      ),
      body: RefreshIndicator(
        onRefresh: _refreshTeams,
        child: Container(
          color: Colors.grey[200], // background color
          child: ListView.builder(
            itemCount: _teams.length,
            itemBuilder: (context, index) {
              final team = _teams[index];
              return Card(
                child: ListTile(
                  title: Text(
                    team.name,
                    style: TextStyle(fontSize: 24), // bigger title
                  ),
                  subtitle: Text('Players: ${team.players.join('\n')} '),
                  trailing: _currentTeam == team.name
                      ? IconButton(
                          icon: Icon(Icons.clear),
                          onPressed: () {
                            _leaveTeam();
                          },
                        )
                      : IconButton(
                          icon: Icon(Icons.add),
                          onPressed: () {
                            _joinTeam(team.name);
                          },
                        ),
                ),
              );
            },
          ),
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
                onPressed: () {
                  if (_currentTeam != null) {
                    // Prompt user to leave current team
                    _showLeaveCurrentTeamDialog();
                    return;
                  }
                  _showAddTeamDialog(context);
                },
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
                onPressed: () async {
                  if (_currentTeam == null) {
                    // prompt the user to join a team before readying
                    showDialog(
                      context: context,
                      builder: (context) {
                        return AlertDialog(
                          title: Text('Join a Team'),
                          content: Text(
                              'You need to join a team before you can ready up.'),
                          actions: [
                            TextButton(
                              onPressed: () {
                                Navigator.of(context).pop();
                              },
                              child: Text('OK'),
                            ),
                          ],
                        );
                      },
                    );
                    return;
                  }
                  setState(() {
                    _isReady = !_isReady;
                  });

                  // send a post request to the server to set the player as ready

                  String token = globals.jwt;

                  final headers = {
                    'Authorization': 'Bearer $token',
                  };

                  final response = await http.post(
                    Uri.parse('https://${globals.server_address}/ready'),
                    headers: headers,
                  );

                  await _refreshTeams();
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
