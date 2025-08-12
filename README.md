# black

A blazing-fast CLI tool to calculate the percentage of black pixels in images.

## ✨ Features

- 🚀 **Lightning Fast**: Uses Rayon for parallel processing
- 📁 **Batch Processing**: Process entire directories at once
- 🎨 **Beautiful Output**: Color-coded results with filename highlighting
- 🔍 **Smart Detection**: Automatically detects image files
- 💪 **Robust**: Handles various image formats (PNG, JPG, JPEG, GIF, BMP, TIFF, WEBP)

## 🚀 Installation

```bash
# Clone the repository
git clone https://github.com/luhann/black.git
cd black

# Build and install
cargo install --path .
```

## 📖 Usage

### Single Image
```bash
black image.jpg
```
Output:
```
image.jpg: 23.45%
```

### Directory Processing
```bash
black --directory /path/to/images/
```
Output:
```
photo1.jpg: 12.34%
nature.png: 45.67%
artwork.jpeg: 78.90%
```

### Command Line Options
```bash
black [OPTIONS] <PATH>

Arguments:
  <PATH>  File path of the image or directory

Options:
  -d, --directory  Process a whole directory
  -h, --help       Print help
  -V, --version    Print version information
```

## 🎯 Examples

```bash
# Analyze a single image
black sunset.jpg
# Output: sunset.jpg: 15.42%

# Process all images in a directory
black --directory ./photos
# Output:
# beach.jpg: 8.23%
# mountain.png: 34.56%
# city.jpeg: 67.89%

# Process current directory
black --directory .
```

## 🏗️ How It Works

Black analyzes each pixel in an image and determines what percentage are "black" pixels. The tool:

1. 🔍 **Loads** the image using efficient image processing libraries
2. ⚡ **Processes** pixels in parallel for maximum speed
3. 🧮 **Calculates** the percentage of black pixels
4. 🎨 **Displays** results with beautiful formatting

## 📊 Performance

Black is optimized for speed:
- ⚡ Parallel pixel processing for single images
- 🔄 Concurrent processing of multiple files in directories
- 🎯 Efficient memory usage
- 📈 Scales with available CPU cores

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. 🍴 Fork the repository
2. 🌿 Create a feature branch
3. ✨ Make your changes
4. 🧪 Add tests if applicable
5. 📝 Update documentation
6. 🚀 Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Powered by the amazing Rust ecosystem
- Thanks to the Rayon team for excellent parallel processing

---

**Made with 🖤 by [luhann](https://github.com/luhann)**
