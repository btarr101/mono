#!/usr/bin/env python3
"""
Script to revert files to their 'before' state from a diff JSON.
Usage: python3 revert_files.py [diff_file.json]
"""

import json
import sys
import os
from pathlib import Path


def revert_files(diff_data):
    """
    Revert files to their 'before' state based on diff data.
    
    Args:
        diff_data: List of dicts containing file changes with 'file', 'before', and 'after' keys
    """
    reverted_count = 0
    deleted_count = 0
    errors = []
    
    for change in diff_data:
        file_path = change.get('file')
        before_content = change.get('before')
        after_content = change.get('after')
        
        if not file_path:
            errors.append(f"Missing file path in change: {change}")
            continue
        
        # Handle file deletions (before is empty, after has content)
        if before_content == "" and after_content != "":
            if os.path.exists(file_path):
                try:
                    os.remove(file_path)
                    deleted_count += 1
                    print(f"✓ Deleted: {file_path}")
                except Exception as e:
                    errors.append(f"Failed to delete {file_path}: {e}")
            else:
                print(f"⊘ Already deleted: {file_path}")
            continue
        
        # Handle file creation/modification
        if before_content is not None:
            try:
                # Create parent directories if they don't exist
                Path(file_path).parent.mkdir(parents=True, exist_ok=True)
                
                # Write the before content
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(before_content)
                
                reverted_count += 1
                additions = change.get('additions', 0)
                deletions = change.get('deletions', 0)
                print(f"✓ Reverted: {file_path} (undid +{additions}/-{deletions})")
                
            except Exception as e:
                errors.append(f"Failed to revert {file_path}: {e}")
        else:
            print(f"⊘ Skipped: {file_path} (no before content)")
    
    # Print summary
    print("\n" + "="*60)
    print(f"Summary:")
    print(f"  Files reverted: {reverted_count}")
    print(f"  Files deleted: {deleted_count}")
    print(f"  Errors: {len(errors)}")
    print("="*60)
    
    if errors:
        print("\nErrors encountered:")
        for error in errors:
            print(f"  ✗ {error}")
        return False
    
    return True


def main():
    # Check if diff file is provided as argument
    if len(sys.argv) > 1:
        diff_file = sys.argv[1]
    else:
        # Default to looking for a diff.json in current directory
        diff_file = "diff.json"
    
    # Read the diff data
    try:
        with open(diff_file, 'r', encoding='utf-8') as f:
            diff_data = json.load(f)
    except FileNotFoundError:
        print(f"Error: Diff file '{diff_file}' not found.")
        print(f"Usage: python3 {sys.argv[0]} [diff_file.json]")
        sys.exit(1)
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in '{diff_file}': {e}")
        sys.exit(1)
    
    # Confirm with user
    print(f"About to revert {len(diff_data)} files to their 'before' state.")
    print(f"Current directory: {os.getcwd()}")
    response = input("Continue? (yes/no): ").strip().lower()
    
    if response not in ['yes', 'y']:
        print("Aborted.")
        sys.exit(0)
    
    # Perform the revert
    success = revert_files(diff_data)
    
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()