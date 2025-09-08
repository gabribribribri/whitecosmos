# This AI generated script (flemme de l'écrire moi même) translates a text file containing string representations of
# Space, Tab, and Line-Feed characters into their actual ASCII counterparts.

import os
import sys

def translate_file(translation, input_filepath, output_filepath):
    """
    Reads a file with specific character representations and translates them.

    Args:
        input_filepath (str): The path to the input file.
        output_filepath (str): The path to the output file where the
                               translated content will be saved.
    """
    # Check if the input file exists
    if not os.path.exists(input_filepath):
        print(f"Error: The input file '{input_filepath}' was not found.")
        return

    try:
        # Read the content of the input file
        with open(input_filepath, 'r', encoding='utf-8') as infile:
            content = infile.read()

        # Perform the replacements
        if translation == "wws-fws" :
            translated_content = content.replace('[LF]', 'l')
            translated_content = translated_content.replace('[Space]', 's')
            translated_content = translated_content.replace('[Tab]', 't')
        # elif translation == "fws-ws" :
        #     translated_content = content.replace('l', '\n')
        #     translated_content = translated_content.replace('s', ' ')
        #     translated_content = translated_content.replace('t', '\t')
        elif translation == "ws-fws" :
            translated_content = content.replace('\n', 'l')
            translated_content = translated_content.replace(' ', 's')
            translated_content = translated_content.replace('\t', 't')
        # elif translation == "wws-ws" :
        #     translated_content = content.replace('[LF]', '\n')
        #     translated_content = translated_content.replace('[Space]', ' ')
        #     translated_content = translated_content.replace('[Tab]', '\t')
        else :
            raise "Not implemented for now"


        # Write the translated content to the new output file
        with open(output_filepath, 'w', encoding='utf-8') as outfile:
            outfile.write(translated_content)

        print(f"Successfully translated '{input_filepath}' to '{output_filepath}'.")
        print("The translated content has been written to the output file.")

    except Exception as e:
        print(f"An error occurred during translation: {e}")

if __name__ == "__main__":
    # Define the file paths
    translation = sys.argv[1]
    input_file = sys.argv[2]
    output_file = sys.argv[3]

    translate_file(translation, input_file, output_file)
