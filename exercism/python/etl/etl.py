def transform(legacy_data):
    return {c.lower(): value
            for value, chars in legacy_data.items()
            for c in chars}
