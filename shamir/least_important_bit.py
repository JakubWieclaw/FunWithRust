from PIL import Image  # Import the PIL library to handle image operations

def encode_message(image_path, message, output_image_path):
    image = Image.open(image_path)  # Load the image
    pixels = list(image.getdata())  # Get pixel data
    
    # Convert the message to binary and append end-of-message marker
    binary_message = ''.join(format(ord(char), '08b') for char in message)
    binary_message += '1111111111111110'
    
    new_pixels = []
    message_index = 0
    
    for pixel in pixels:
        if message_index < len(binary_message):
            new_pixel = []
            for color in pixel[:3]:  # Modify the RGB values
                if message_index < len(binary_message):
                    new_pixel.append(color & ~1 | int(binary_message[message_index]))
                    message_index += 1
                else:
                    new_pixel.append(color)
            if len(pixel) == 4:  # Preserve the alpha channel if present
                new_pixel.append(pixel[3])
            new_pixels.append(tuple(new_pixel))
        else:
            new_pixels.append(pixel)
    
    # Create and save a new image with the modified pixels
    new_image = Image.new(image.mode, image.size)
    new_image.putdata(new_pixels)
    new_image.save(output_image_path)

def decode_message(image_path):
    image = Image.open(image_path)  # Load the image
    pixels = list(image.getdata())  # Get pixel data
    
    binary_message = ''
    for pixel in pixels:
        for color in pixel[:3]:  # Read the RGB values
            binary_message += str(color & 1)  # Extract the least significant bit
    
    message = ''
    for i in range(0, len(binary_message), 8):  # Process 8 bits at a time
        byte = binary_message[i:i+8]
        if byte == '11111110':  # Check for end-of-message marker
            break
        message += chr(int(byte, 2))  # Convert binary to character
    
    return message

if __name__ == "__main__":
    input_image_path = 'bmp_3.bmp'  # Path to the input image
    output_image_path = 'output.bmp'  # Path to save the output image
    message = "Hidden message"  # The message to encode
    
    encode_message(input_image_path, message, output_image_path)  # Encode the message
    print(f"Encoded message: {message}")
    
    decoded_message = decode_message(output_image_path)  # Decode the message
    print(f"Decoded message: {decoded_message}")
