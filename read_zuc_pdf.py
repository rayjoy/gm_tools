import pypdf
import os

try:
    reader = pypdf.PdfReader("ZUC-256新初始化方案英文版.pdf")
    text = ""
    for page in reader.pages:
        text += page.extract_text() + "\n"
    print(text)
except Exception as e:
    print(f"Error: {e}")
