import 'package:flutter/material.dart';

class MapPage extends StatefulWidget {
  @override
  _MapPageState createState() => _MapPageState();
}

class _MapPageState extends State<MapPage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Interactive Map'),
      ),
      body: Center(
        child: InteractiveViewer(
          minScale: 0.1,
          maxScale: 4.0,
          child: Stack(
            children: [
              Image.network(
                'https://example.com/your-image.jpg',
                width: 2000,
                height: 2000,
                fit: BoxFit.cover,
              ),
              Positioned(
                left: 100, // X coordinate of the marker
                top: 100, // Y coordinate of the marker
                child: Icon(
                  Icons.location_on,
                  color: Colors.red,
                  size: 30,
                ),
              ),
              // Add more Positioned widgets for additional markers
            ],
          ),
        ),
      ),
    );
  }
}
