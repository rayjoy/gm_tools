from pypdf import PdfReader
import sys

def extract_text(pdf_path):
    try:
        reader = PdfReader(pdf_path)
        print(f"Number of Pages: {len(reader.pages)}")
        text = ""
        for i, page in enumerate(reader.pages):
            page_text = page.extract_text()
            print(f"--- Page {i+1} ---")
            print(page_text)
            text += page_text + "\n"
        return text
    except ImportError:
        print("Error: pypdf is not installed.")
    except Exception as e:
        print(f"Error reading PDF: {e}")

if __name__ == "__main__":
    pdf_path = r"d:\ray\source\copilot\gm_tools\ZUC-256新初始化方案英文版.pdf"
    extract_text(pdf_path)
