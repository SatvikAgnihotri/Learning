#comments in python
import re
from bs4 import BeautifulSoup
from urllib.request import urlopen
url_to_scrape = "https://www.scholarships.com/financial-aid/college-scholarships/scholarships-by-grade-level/high-school-scholarships/"

request_page = urlopen(url_to_scrape)
page_html = request_page.read()
request_page.close()

html_soup = BeautifulSoup(page_html, 'html.parser')

scholarship_items = html_soup.find_all('a')
for a in scholarship_items:
     print(a.get_text())

descriptions = html_soup.find_all('li')
for a in descriptions:
     print(a.get_text())

#change -1 to 1 if scholarship items is longer
n = range(-1 * (len(scholarship_items) - (len(descriptions))))
for i in n:
     scholarship_items.append(' ')

print(len(scholarship_items))
print(len(descriptions))




 
#Convert lists to CSV
import pandas as pd
dict = {"Scholarship Name": scholarship_items, "Descriptions": descriptions}
df = pd.DataFrame(dict)
df.to_csv('Scholarships.csv')



""" page = urlopen(url)
html_bytes = page.read()
html = html_bytes.decode("utf-8")
soup = BeautifulSoup(html, "html.parser") """


# div = "div"
# n = 0

# for "div" in html:
#     n += n+1

# print(n)