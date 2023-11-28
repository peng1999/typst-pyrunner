#import "@local/pyrunner:0.0.1": python

#python(```
import re

string = "My email address is john.doe@example.com and my friend's email address is jane.doe@example.net."

re.findall(r"\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b", string)
```)

#python(```
import sys
sys.version
```)

#python(```
[1, True, {"a": 3, 5: "hello"}, None]
```)
