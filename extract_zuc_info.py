import sys

try:
    from pypdf import PdfReader
except ImportError:
    print("Error: 'pypdf' library is not installed.")
    print("Please install it running: pip install pypdf")
    sys.exit(1)

def extract_zuc_details(pdf_path):
    print(f"Reading file: {pdf_path}")
    try:
        reader = PdfReader(pdf_path)
        print(f"Total Pages: {len(reader.pages)}")
        
        full_text = ""
        for i, page in enumerate(reader.pages):
            page_text = page.extract_text()
            full_text += f"\n--- Page {i+1} ---\n{page_text}"
        
        print("\n=== EXTRACTED TEXT SEARCH ===")
        
        # 1. Look for Constants D1, D2
        search_terms = ["D1", "D2", "d1", "d2"]
        found_constants = False
        print("\n[1] Searching for Constants (D1, D2):")
        lines = full_text.split('\n')
        for i, line in enumerate(lines):
            for term in search_terms:
                if term in line and ("constant" in line.lower() or "=" in line):
                    print(f"  Found '{term}' on line {i+1}: {line.strip()}")
                    # Print context
                    for j in range(max(0, i-2), min(len(lines), i+3)):
                        if i != j:
                            print(f"    {lines[j].strip()}")
                    found_constants = True
                    break
        if not found_constants:
            print("  No explicit definitions of D1/D2 found in text search.")

        # 2. Loading Key Process
        print("\n[2] Searching for 'Loading Key' process:")
        found_loading = False
        for i, line in enumerate(lines):
            if "load" in line.lower() and "key" in line.lower():
                print(f"  Found mention on line {i+1}: {line.strip()}")
                # Context
                for j in range(i, min(len(lines), i+10)): # Print next 10 lines for process steps
                     print(f"    {lines[j].strip()}")
                found_loading = True
                print("    ...")
        if not found_loading:
             print("  No specific 'Loading Key' section found.")

        # 3. Differences
        print("\n[3] Searching for Differences/Old Scheme:")
        for i, line in enumerate(lines):
            if "differ" in line.lower() or "old" in line.lower() or "replace" in line.lower():
                 print(f"  Line {i+1}: {line.strip()}")

        # 4. Pseudocode/Steps
        print("\n[4] Searching for Steps/Pseudocode:")
        for i, line in enumerate(lines):
            if "step" in line.lower() or "1." in line or "2." in line:
                # Naive check for numbered list which implies steps
                 pass # Too noisy to print all, rely on full dump if needed.
        
        print("\n=== FULL EXTRACTED TEXT for Manual Review ===")
        # Print the full text at the end so the user can verify
        print(full_text)
        
    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    # Use raw string for path to avoid escape issue
    target_path = r"d:\ray\source\copilot\gm_tools\ZUC-256新初始化方案英文版.pdf"
    extract_zuc_details(target_path)
