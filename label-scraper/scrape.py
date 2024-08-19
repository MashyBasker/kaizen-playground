#!/usr/bin/python3

from bs4 import BeautifulSoup
import requests

BASE_URL = "https://github.com/numpy/numpy/labels"

def scrape_url(url):
    res = requests.get(url)
    if res.status_code == 200:
        html_content = res.text
    else:
        print(f"Failed to get the page. Response: ", res.status_code)
    
    soup = BeautifulSoup(html_content, 'html.parser')
    label_elem = soup.find_all('div', class_="js-label-preview")
    label_with_desc = []
    for label in label_elem:
        labels = label.find_all('span', class_='IssueLabel')
        for l in labels:
            name = l.text.strip()
            desc_elem = l.find_next('div')
            desc_str = desc_elem.text.strip()
            label_desc = desc_str if desc_str else "N/A"
            label_with_desc.append((name, label_desc))
    return label_with_desc
    
if __name__ == "__main__":
    all_labels = []
    page_num = 1
    while True:
        print(f"Scraping page {page_num}...")
        url = f"{BASE_URL}?page={page_num}"
        _labels = scrape_url(url)
        if not _labels:
            break
        all_labels += _labels
        page_num += 1
    for x in all_labels:
        print(f"[+] Label: {x[0]}\tDescription: {x[1]}")
    print("[+] Total number of labels: ", len(all_labels))
