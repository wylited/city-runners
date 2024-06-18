import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:geolocator/geolocator.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'globals.dart' as globals;
import 'server.dart';
import 'login.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initializeGlobals();
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'CityRunners',
      themeMode: ThemeMode.system,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: globals.initialized
          ? const ServerSelectionPage()
          : const LoadingScreen(),
    );
  }
}

class LoadingScreen extends StatelessWidget {
  const LoadingScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: CircularProgressIndicator(),
      ),
    );
  }
}

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(' App'),
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
