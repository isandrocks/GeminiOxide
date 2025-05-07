import google.generativeai as genai
import os

# Configure the API key.
# It's recommended to set this as an environment variable.
# For example, os.environ.get("GEMINI_API_KEY")
# Replace "YOUR_API_KEY" with your actual API key if not using an environment variable.
try:
    genai.configure(api_key=os.environ["GEMINI_API_KEY"])
except KeyError:
    print("Error: GEMINI_API_KEY environment variable not set.")
    print("Please set it or replace os.environ[\"GEMINI_API_KEY\"] with your actual key.")
    exit()


prompt = "dfgsdfI made a  transparent background customizable Vector of Miku from Mimukauwa Nice Try but it wont let me post it anywhere as a SVG"

# Initialize the model
# Updated to use a current valid model name like 'gemini-1.5-flash'
# 'gemini-2.0-flash' is not a recognized model name as of my last update.
# Please check the official Google AI documentation for the latest model names.
# Using 'gemini-1.5-flash' as a placeholder.
try:
    model = genai.GenerativeModel('gemini-1.5-flash') # Or 'gemini-pro' or other suitable model
    response = model.generate_content(prompt)
    text = response.text
    print(text)
except Exception as e:
    print(f"An error occurred: {e}")
    if hasattr(response, 'prompt_feedback'):
        print(f"Prompt Feedback: {response.prompt_feedback}")

