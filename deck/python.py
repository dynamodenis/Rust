def truncate_url(url, max_length=255):
    """Truncates the URL if it exceeds the max_length."""
    if len(url) > max_length:
        truncated_url = url[:max_length]
        print(f"URL exceeded {max_length} characters. Truncated URL:\n{truncated_url}")
        return truncated_url
    else:
        print("URL is within the allowed length.")
        return url

# Example usage
url = "https://www.example.com/" + "a" * 300  # Creating a long URL
result = truncate_url(url)