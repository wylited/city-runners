import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:geolocator/geolocator.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'globals.dart' as globals;
import 'server.dart';
import 'main.dart';

class LoginPage extends StatefulWidget {
  const LoginPage({super.key});

  @override
  _LoginPageState createState() => _LoginPageState();
}

class _LoginPageState extends State<LoginPage> {
  final _formKey = GlobalKey<FormState>();
  String? _username;
  String? _password;
  String? _jwt;
  String? _serverAddress;
  bool _initialized = false;

  @override
  void initState() {
    super.initState();
    setState(() {
      _username = globals.username;
      _password = globals.password;
      _jwt = globals.jwt;
      _serverAddress = globals.server_address;
    });

    setState(() {
      _initialized = true;
    });
  }

  Future<void> _saveCredentials(
      String username, String password, String jwtToken) async {
    globals.username = username;
    globals.password = password;
    globals.jwt = jwtToken;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('username', username);
    await prefs.setString('password', password);
    await prefs.setString('jwt', jwtToken);
  }

  Future<String?> _login() async {
    final headers = {
      'Content-Type': 'application/json',
    };

    final body = {
      'username': _username,
      'password': _password,
    };

    final jsonBody = jsonEncode(body);

    try {
      final response = await http.post(
        Uri.parse('https://$_serverAddress/login'),
        headers: headers,
        body: jsonBody,
      );

      if (response.statusCode == 202) {
        final jsonData = jsonDecode(response.body);
        return jsonData['token'];
      } else {
        print('failed at login');
        return null;
      }
    } catch (e) {
      print('Error: $e');
      return null;
    }
  }

  Future<bool> _validateToken(String token) async {
    final url = Uri.parse('https://$_serverAddress/validate');
    try {
      final response = await http.get(
        url,
        headers: {
          'Authorization': 'Bearer $token',
        },
      );
      if (response.statusCode == 200) {
        return true;
      }
      return false;
      print('failed at validating token');
    } catch (e) {
      print('error $e');
      return false;
    }
  }

  @override
  Widget build(BuildContext context) {
    if (!_initialized) {
      return Scaffold(
        body: Center(
          child: CircularProgressIndicator(),
        ),
      );
    }
    return Scaffold(
      appBar: AppBar(
        title: Text('Login'),
      ),
      body: Padding(
        padding: EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              TextFormField(
                initialValue: _username,
                onChanged: (value) {
                  setState(() {
                    _username = value;
                  });
                },
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a valid username';
                  }
                  return null;
                },
                decoration: InputDecoration(
                  labelText: 'Username',
                  border: OutlineInputBorder(),
                ),
              ),
              SizedBox(height: 16.0),
              TextFormField(
                initialValue: _password,
                onChanged: (value) {
                  setState(() {
                    _password = value;
                  });
                },
                obscureText: true,
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter a valid password';
                  }
                  return null;
                },
                decoration: InputDecoration(
                  labelText: 'Password',
                  border: OutlineInputBorder(),
                ),
              ),
              SizedBox(height: 16.0),
              ElevatedButton(
                onPressed: () async {
                  if (_formKey.currentState?.validate() ?? false) {
                    String? token = await _login();
                    if (token != null && await _validateToken(token)) {
                      await _saveCredentials(_username!, _password!, token);
                      Navigator.of(context).pushReplacement(
                        MaterialPageRoute(builder: (_) => HomePage()),
                      );
                    } else {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(
                          content: Text('Invalid login credentials'),
                        ),
                      );
                    }
                  }
                },
                child: Text('Login'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
