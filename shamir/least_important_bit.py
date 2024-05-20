from PIL import Image

def encode_message(image_path, message, output_image_path):
    image = Image.open(image_path)
    pixels = list(image.getdata())
    
    # Zamień wiadomość na bity
    binary_message = ''.join(format(ord(char), '08b') for char in message)
    binary_message += '1111111111111110'
    
    new_pixels = []
    message_index = 0
    for pixel in pixels:
        if message_index < len(binary_message):
            new_pixel = []
            for color in pixel[:3]:
                if message_index < len(binary_message):
                    new_pixel.append(color & ~1 | int(binary_message[message_index]))
                    message_index += 1
                else:
                    new_pixel.append(color)
            if len(pixel) == 4:
                new_pixel.append(pixel[3])
            new_pixels.append(tuple(new_pixel))
        else:
            new_pixels.append(pixel)
    
    new_image = Image.new(image.mode, image.size)
    new_image.putdata(new_pixels)
    new_image.save(output_image_path)

def decode_message(image_path):
    image = Image.open(image_path)
    pixels = list(image.getdata())
    
    binary_message = ''
    for pixel in pixels:
        for color in pixel[:3]:
            binary_message += str(color & 1)
    
    message = ''
    for i in range(0, len(binary_message), 8):
        byte = binary_message[i:i+8]
        if byte == '11111110':
            break
        message += chr(int(byte, 2))
    
    return message

if __name__ == "__main__":
    input_image_path = 'bmp_3.bmp'
    output_image_path = 'output.bmp'
    message = "Hidden message"
    
    encode_message(input_image_path, message, output_image_path)
    print(f"Encoded message: {message}")
    
    decoded_message = decode_message(output_image_path)
    print(f"Decoded message: {decoded_message}")
