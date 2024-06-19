import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:geolocator/geolocator.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'globals.dart' as globals;
import 'server.dart';
import 'login.dart';
import 'main.dart';

class ServerSelectionPage extends StatefulWidget {
  const ServerSelectionPage();
  @override
  _ServerSelectionPageState createState() => _ServerSelectionPageState();
}

class _ServerSelectionPageState extends State<ServerSelectionPage> {
  final _formKey = GlobalKey<FormState>();
  String? _serverAddress;

  @override
  void initState() {
    super.initState();
    setState(() {
      _serverAddress = globals.server_address;
    });
  }

  Future<void> _saveServerAddress(String serverAddress) async {
    globals.server_address = _serverAddress ?? globals.server_address;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('server_address', serverAddress);
  }

  Future<bool> _validateServerAddress(String serverAddress) async {
    final url = Uri.parse('https://$serverAddress/');
    try {
      final response = await http.get(url);
      return response.statusCode == 200;
    } catch (e) {
      return false;
    }
  }

  @override
  Widget build(BuildContext contex) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Server Selection'),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              TextFormField(
                initialValue: _serverAddress,
                onChanged: (value) {
                  setState(() {
                    _serverAddress = value;
                  });
                },
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a valid server address';
                  }
                  if (!Uri.parse('https://$value/').isAbsolute) {
                    return 'Invalid address format';
                  }
                  return null;
                },
                decoration: InputDecoration(
                  labelText: 'Server Address',
                  border: OutlineInputBorder(),
                ),
              ),
              SizedBox(height: 16.0),
              ElevatedButton(
                onPressed: () async {
                  if (_formKey.currentState?.validate() ?? false) {
                    if (await _validateServerAddress(_serverAddress!)) {
                      await _saveServerAddress(_serverAddress!);
                      Navigator.of(context).pushReplacement(
                        MaterialPageRoute(builder: (_) => LoginPage()),
                      );
                    } else {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text('Invalid server address'),
                        ),
                      );
                    }
                  }
                },
                child: Text('Save and Continue'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
