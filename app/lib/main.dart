import 'package:geolocator/geolocator.dart';
import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'dart:convert';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'CityRunners',
      themeMode: ThemeMode.system, // Use system theme mode
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const ServerSelectionPage(),
    );
  }
}

class ServerSelectionPage extends StatefulWidget {
  const ServerSelectionPage();
  @override
  _ServerSelectionPageState createState() => _ServerSelectionPageState();
}

class _ServerSelectionPageState extends State<ServerSelectionPage> {
  final _formKey = GlobalKey<FormState>();
  String _serverAddress = '';

  @override
  void initState() {
    super.initState();
    _loadServerAddress();
  }

  Future<void> _loadServerAddress() async {
    final prefs = await SharedPreferences.getInstance();
    final storedAddress = prefs.getString('server_address');
    if (storedAddress != null) {
      setState(() {
        _serverAddress = storedAddress;
      });
    }
  }

  Future<void> _saveServerAddress() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('server_address', _serverAddress);
  }

  Future<bool> _validateServerAddress() async {
    final url = Uri.parse('$_serverAddress/');
    print('server address $_serverAddress');
    try {
      final response = await http.get(url);
      return response.statusCode == 200;
    } catch (e) {
      print('except $e');
      return false;
    }
  }

  @override
  Widget build(BuildContext context) {
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
                  if (!Uri.parse(value).isAbsolute) {
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
                    if (await _validateServerAddress()) {
                      await _saveServerAddress();
                      Navigator.of(context).pushReplacement(
                        MaterialPageRoute(builder: (_) => MyHomePage()),
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

class ServerSelectionPage extends StatefulWidget {
  const ServerSelectionPage();
  @override
  _ServerSelectionPageState createState() => _ServerSelectionPageState();
}

class _ServerSelectionPageState extends State<ServerSelectionPage> {
  final _formKey = GlobalKey<FormState>();
  String _serverAddress = '';

  @override
  void initState() {
    super.initState();
    _loadServerAddress();
  }

  Future<void> _loadServerAddress() async {
    final prefs = await SharedPreferences.getInstance();
    final storedAddress = prefs.getString('server_address');
    if (storedAddress != null) {
      setState(() {
        _serverAddress = storedAddress;
      });
    }
  }

  Future<void> _saveServerAddress() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString('server_address', _serverAddress);
  }

  Future<bool> _validateServerAddress() async {
    final url = Uri.parse('$_serverAddress/');
    print('server address $_serverAddress');
    try {
      final response = await http.get(url);
      return response.statusCode == 200;
    } catch (e) {
      print('except $e');
      return false;
    }
  }

  @override
  Widget build(BuildContext context) {
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
                  if (!Uri.parse(value).isAbsolute) {
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
                    if (await _validateServerAddress()) {
                      await _saveServerAddress();
                      Navigator.of(context).pushReplacement(
                        MaterialPageRoute(builder: (_) => MyHomePage()),
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

class MyHomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('My App'),
      ),
      body: Center(
        child: Text('Hello, World!'),
      ),
    );
  }
}

class LocationSender extends StatefulWidget {
  final String serverAddress;

  const LocationSender({super.key, required this.serverAddress});

  @override
  State<LocationSender> createState() => _LocationSenderState();
}

class _LocationSenderState extends State<LocationSender> {
  int _counter = 0;

  Future<void> _sendLocationToServer() async {
    // Get the device's current location
    Position position = await Geolocator.getCurrentPosition(
        desiredAccuracy: LocationAccuracy.high);

    // Create a JSON payload with the location data
    Map<String, dynamic> locationData = {
      'latitude': position.latitude,
      'longitude': position.longitude,
    };

    // Send the location data to the API server
    final Uri url = Uri.parse(widget.serverAddress + '/location');
    final response = await http.post(
      url,
      headers: {
        'Content-Type': 'application/json',
      },
      body: jsonEncode(locationData),
    );

    // Check the response status code
    if (response.statusCode == 200) {
      print('Location sent successfully!');
    } else {
      print('Error sending location: ${response.statusCode}');
    }
  }

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
    _sendLocationToServer(); // Call the function to send the location
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Location Sender'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            ElevatedButton(
              onPressed: _incrementCounter,
              child: const Text('Send Location'),
            ),
          ],
        ),
      ),
    );
  }
}
