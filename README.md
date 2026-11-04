# Tor Site Finder

Rust CLI that scrapes Tor .onion sites from multiple search engines by keyword


<img width="1280" height="640" alt="github-banner" src="https://github.com/user-attachments/assets/d6e578e0-cad9-437b-9c3b-f6eb4bbd6686" />
<svg xmlns="http://www.w3.org/2000/svg" width="1280" height="640" viewBox="0 0 1280 640" xmlns:c2pa="http://c2pa.org/manifest"><metadata><c2pa:manifest>AAAWgmp1bWIAAAAeanVtZGMycGEAEQAQgAAAqgA4m3EDYzJwYQAAABZcanVtYgAAAEdqdW1kYzJtYQARABCAAACqADibcQN1cm46YzJwYTplMmI5ZmE1NS0yMmY2LTRkMjMtODRiMy1hMTI3OGYyMmMzZDcAAAADl2p1bWIAAAApanVtZGMyYXMAEQAQgAAAqgA4m3EDYzJwYS5hc3NlcnRpb25zAAAAALxqdW1iAAAARGp1bWRjYm9yABEAEIAAAKoAOJtxE2MycGEuaW5ncmVkaWVudC52MwAAAAAYYzJzaHTGpBaCAlBecUELGkaAXQUAAABwY2JvcqNpZGM6Zm9ybWF0bWltYWdlL3N2Zyt4bWxqaW5zdGFuY2VJRHgseG1wOmlpZDpiZWE0MmZmYS1lZjJhLTQ5ZjctOWFmMy1lNzI2ODg5N2JjOGVscmVsYXRpb25zaGlwaHBhcmVudE9mAAAB4mp1bWIAAABBanVtZGNib3IAEQAQgAAAqgA4m3ETYzJwYS5hY3Rpb25zLnYyAAAAABhjMnNoqSQLBMRRvLlY71xAuHfsbQAAAZljYm9yomdhY3Rpb25zgqJmYWN0aW9ua2MycGEub3BlbmVkanBhcmFtZXRlcnOha2luZ3JlZGllbnRzgaJjdXJseC1zZWxmI2p1bWJmPWMycGEuYXNzZXJ0aW9ucy9jMnBhLmluZ3JlZGllbnQudjNkaGFzaFggwAxebK26EFOZn+RB1D1dSStmv5ymsA/myVmOgrZRcMKkZmFjdGlvbngdY29tLmFudGhyb3BpYy5jbGF1ZGUucHJvdmlkZWRqcGFyYW1ldGVyc6F4H2NvbS5hbnRocm9waWMub3JpZ2luLWNvbmZpZGVuY2VndW5rbm93bmtkZXNjcmlwdGlvbnhmQ2xhdWRlIHByb3ZpZGVkIHRoaXMgZmlsZSBhdCB0aGUgcmVxdWVzdCBvZiBhIHVzZXIgYW5kIG1heSBoYXZlIGNyZWF0ZWQgb3IgbW9kaWZpZWQgdGhlIGZpbGUgY29udGVudHMubXNvZnR3YXJlQWdlbnShZG5hbWVmQ2xhdWRlcmFsbEFjdGlvbnNJbmNsdWRlZPUAAADIanVtYgAAAEBqdW1kY2JvcgARABCAAACqADibcRNjMnBhLmhhc2guZGF0YQAAAAAYYzJzaG/qgk9wZtkOclp7p3WaV/cAAACAY2JvcqVjYWxnZnNoYTI1NmNwYWRNAAAAAAAAAAAAAAAAAGRoYXNoWCDGvgEHjaClFx4AciNVIF/8NaXKU89uBHaBAMAwqUEmE2RuYW1lbmp1bWJmIG1hbmlmZXN0amV4Y2x1c2lvbnOBomVzdGFydBiYZmxlbmd0aBkeBAAAAj5qdW1iAAAAJ2p1bWRjMmNsABEAEIAAAKoAOJtxA2MycGEuY2xhaW0udjIAAAACD2Nib3KlY2FsZ2ZzaGEyNTZpc2lnbmF0dXJleE1zZWxmI2p1bWJmPS9jMnBhL3VybjpjMnBhOmUyYjlmYTU1LTIyZjYtNGQyMy04NGIzLWExMjc4ZjIyYzNkNy9jMnBhLnNpZ25hdHVyZWppbnN0YW5jZUlEeCx4bXA6aWlkOmQwZmQ0NTMyLTNjODYtNDUxZS04YTk2LTFiOWNmZmMxMDRmZXJjcmVhdGVkX2Fzc2VydGlvbnODomN1cmx4LXNlbGYjanVtYmY9YzJwYS5hc3NlcnRpb25zL2MycGEuaW5ncmVkaWVudC52M2RoYXNoWCDADF5srboQU5mf5EHUPV1JK2a/nKawD+bJWY6CtlFwwqJjdXJseCpzZWxmI2p1bWJmPWMycGEuYXNzZXJ0aW9ucy9jMnBhLmFjdGlvbnMudjJkaGFzaFggNwy/UUplWy31/ObLP7MPMFAvYayaWhjkZWUYo2vPz3+iY3VybHgpc2VsZiNqdW1iZj1jMnBhLmFzc2VydGlvbnMvYzJwYS5oYXNoLmRhdGFkaGFzaFgg9Q8QBN4jdTHwHglgZ11IUwT0gDmIalL3ag4ypYkVDDN0Y2xhaW1fZ2VuZXJhdG9yX2luZm+jZG5hbWVvQW50aHJvcGljIEZpbGVzZ3ZlcnNpb25lMS4wLjBrc3BlY1ZlcnNpb25lMi40LjAAABA4anVtYgAAAChqdW1kYzJjcwARABCAAACqADibcQNjMnBhLnNpZ25hdHVyZQAAABAIY2JvctKEWQISogEmGCFZAgowggIGMIIBjaADAgECAhRA5aAK7sI50L64g/oGQgU9Z1UTADAKBggqhkjOPQQDAzBJMRcwFQYDVQQKEw5BbnRocm9waWMsIFBCQzEuMCwGA1UEAxMlQW50aHJvcGljIENvbnRlbnQgQ3JlZGVudGlhbHMgUm9vdCBDQTAeFw0yNjA4MDcxODQzNTZaFw0yODA4MDYxOTQzNTZaMEQxFzAVBgNVBAoTDkFudGhyb3BpYywgUEJDMSkwJwYDVQQDEyBBbnRocm9waWMgQ2xhdWRlIENvbnRlbnQgU2lnbmluZzBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABJh6CmvLUBgFFNU0vUKlOVtE6djd17L5SuwX0LemFisBM3dkd/3cyjxFA3Qo5S46fX0/ihY0VZ7mfb9KF703t5OjWDBWMA4GA1UdDwEB/wQEAwIHgDAVBgNVHSUEDjAMBgorBgEEAYPoXgIBMAwGA1UdEwEB/wQCMAAwHwYDVR0jBBgwFoAUzlHiBIFOZFsj+OPEz5o+nMHXXMIwCgYIKoZIzj0EAwMDZwAwZAIwMXMdFJ4BetLLVY7ORuE9noqbbAZOZn/aArXyTwFAZfKrPzxF2vPoJNf1+UCdg1XGAjBwX1zd9WGqYkqmL5SFqw1QySjr1zJfpJM9+1rdDwSPLMOPOjKuiXjoU/pUUeG9RwmhY3BhZFkNngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPZYQAkMWjAQad4ACEXBCJFkAMi25amJXTqjKR4c5UV6QTBYkLH2RUl+FYQ0KpIoT629F/tIb0cvT/IY+vyyfiLslmU=</c2pa:manifest></metadata>
  <defs>
    <radialGradient id="glowL" cx="0.12" cy="0.1" r="0.55">
      <stop offset="0" stop-color="#ffd75f" stop-opacity="0.10"/>
      <stop offset="1" stop-color="#ffd75f" stop-opacity="0"/>
    </radialGradient>
    <radialGradient id="glowR" cx="0.85" cy="0.35" r="0.5">
      <stop offset="0" stop-color="#5fffff" stop-opacity="0.10"/>
      <stop offset="1" stop-color="#5fffff" stop-opacity="0"/>
    </radialGradient>
    <radialGradient id="vig" cx="0.5" cy="0.5" r="0.75">
      <stop offset="0.55" stop-color="#000" stop-opacity="0"/>
      <stop offset="1" stop-color="#000" stop-opacity="0.55"/>
    </radialGradient>
    <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
      <path d="M40 0H0V40" fill="none" stroke="#ffffff" stroke-opacity="0.035" stroke-width="1"/>
    </pattern>
    <pattern id="scan" width="4" height="4" patternUnits="userSpaceOnUse">
      <rect width="4" height="1" fill="#000" fill-opacity="0.18"/>
    </pattern>
  </defs>

  <!-- background: near-black, slightly see-through (change fill-opacity to adjust) -->
  <rect width="1280" height="640" fill="#05060b" fill-opacity="0.9"/>
  <rect width="1280" height="640" fill="url(#grid)"/>
  <rect width="1280" height="640" fill="url(#glowL)"/>
  <rect width="1280" height="640" fill="url(#glowR)"/>

  <!-- faint onion layers (Tor) as a ghost mark, like a wallpaper watermark -->
  <g fill="none" stroke="#ffffff" stroke-opacity="0.05" stroke-width="2">
    <circle cx="1090" cy="150" r="60"/>
    <circle cx="1090" cy="150" r="110"/>
    <circle cx="1090" cy="150" r="165"/>
    <circle cx="1090" cy="150" r="225"/>
    <circle cx="1090" cy="150" r="290"/>
  </g>

  <g font-family="DejaVu Sans Mono, Menlo, Consolas, monospace">
    <!-- banner -->
    <g font-size="16">
    <text x="70" y="82.0" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">░▒▓████████▓▒░▒▓██████▓▒░░▒▓███████▓▒░       </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    <text x="70" y="100.6" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    <text x="70" y="119.2" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    <text x="70" y="137.8" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░       </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░</tspan></text>
    <text x="70" y="156.4" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    <text x="70" y="175.0" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░  ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░      </tspan><tspan fill="#5fffff">░▒▓█▓▒░      ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    <text x="70" y="193.6" xml:space="preserve" style="white-space:pre"><tspan fill="#ffd75f">   ░▒▓█▓▒░   ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░      </tspan><tspan fill="#5fffff">░▒▓████████▓▒░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░</tspan></text>
    </g>

    <text x="72" y="238" font-size="20" font-weight="bold" fill="#ffd7af">Finder  ·  search Ahmia, check onion status over Tor</text>

    <!-- table panel -->
    <rect x="70" y="265" width="1140" height="310" rx="10" fill="#000" fill-opacity="0.45" stroke="#ffffff" stroke-opacity="0.14" stroke-width="1.5"/>
    <text x="92" y="302" fill="#af87ff" font-size="16" font-weight="bold">No</text>
    <text x="140" y="302" fill="#ff8700" font-size="16" font-weight="bold">Keyword</text>
    <text x="235" y="302" fill="#ffd75f" font-size="16" font-weight="bold">Link</text>
    <text x="640" y="302" fill="#ff5fff" font-size="16" font-weight="bold">Last Active</text>
    <text x="890" y="302" fill="#ffd7af" font-size="16" font-weight="bold">Time Ago</text>
    <text x="1110" y="302" fill="#ff5f5f" font-size="16" font-weight="bold">Status</text>
    <text x="92" y="348" fill="#ff8700" font-size="15" font-weight="bold">1</text>
    <text x="140" y="348" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="348" fill="#ffd7af" font-size="15" font-weight="bold">http://a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5..</text>
    <text x="640" y="348" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 6:24 a.m.</text>
    <text x="890" y="348" fill="#5fffff" font-size="15" font-weight="bold">2 days, 17 hours</text>
    <text x="1110" y="348" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="381" fill="#ff8700" font-size="15" font-weight="bold">2</text>
    <text x="140" y="381" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="381" fill="#ffd7af" font-size="15" font-weight="bold">http://p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0..</text>
    <text x="640" y="381" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 1:57 a.m.</text>
    <text x="890" y="381" fill="#5fffff" font-size="15" font-weight="bold">2 days, 22 hours</text>
    <text x="1110" y="381" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="414" fill="#ff8700" font-size="15" font-weight="bold">3</text>
    <text x="140" y="414" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="414" fill="#ffd7af" font-size="15" font-weight="bold">http://e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5..</text>
    <text x="640" y="414" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 1:56 a.m.</text>
    <text x="890" y="414" fill="#5fffff" font-size="15" font-weight="bold">2 days, 22 hours</text>
    <text x="1110" y="414" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="447" fill="#ff8700" font-size="15" font-weight="bold">4</text>
    <text x="140" y="447" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="447" fill="#ffd7af" font-size="15" font-weight="bold">http://t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0..</text>
    <text x="640" y="447" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 3, 2026, 8:30 a.m.</text>
    <text x="890" y="447" fill="#5fffff" font-size="15" font-weight="bold">15 hours, 29 minutes</text>
    <text x="1110" y="447" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="480" fill="#ff8700" font-size="15" font-weight="bold">5</text>
    <text x="140" y="480" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="480" fill="#ffd7af" font-size="15" font-weight="bold">http://i1j2k3l4m5n6o7p8q9r0s1t2u3v4w5..</text>
    <text x="640" y="480" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 1:54 a.m.</text>
    <text x="890" y="480" fill="#5fffff" font-size="15" font-weight="bold">2 days, 22 hours</text>
    <text x="1110" y="480" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="513" fill="#ff8700" font-size="15" font-weight="bold">6</text>
    <text x="140" y="513" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="513" fill="#ffd7af" font-size="15" font-weight="bold">http://x6y7z8a9b0c1d2e3f4g5h6i7j8k9l0..</text>
    <text x="640" y="513" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 1:37 a.m.</text>
    <text x="890" y="513" fill="#5fffff" font-size="15" font-weight="bold">2 days, 22 hours</text>
    <text x="1110" y="513" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <text x="92" y="546" fill="#ff8700" font-size="15" font-weight="bold">7</text>
    <text x="140" y="546" fill="#ffffff" font-size="15" font-weight="bold">forum</text>
    <text x="235" y="546" fill="#ffd7af" font-size="15" font-weight="bold">http://m1n2o3p4q5r6s7t8u9v0w1x2y3z4a5..</text>
    <text x="640" y="546" fill="#ff5fff" font-size="15" font-weight="bold">Nov. 1, 2026, 1:49 a.m.</text>
    <text x="890" y="546" fill="#5fffff" font-size="15" font-weight="bold">2 days, 22 hours</text>
    <text x="1110" y="546" fill="#ffd75f" font-size="15" font-weight="bold">200</text>
    <line x1="85" y1="316" x2="1195" y2="316" stroke="#ffffff" stroke-opacity="0.18" stroke-width="1.5"/>
  </g>

  <!-- scanlines + vignette on top -->
  <rect width="1280" height="640" fill="url(#scan)"/>
  <rect width="1280" height="640" fill="url(#vig)"/>
</svg>


# Tor-Site-Finder

Rust CLI that scrapes Tor .onion sites from multiple search engines by keyword
