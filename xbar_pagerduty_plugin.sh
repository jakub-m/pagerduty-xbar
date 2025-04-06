#!/bin/bash

# <xbar.title>PagerDuty status</xbar.title>
# <xbar.version>v0.1.0</xbar.version>
# <xbar.author.github>jakub-m</xbar.author.github>
# <xbar.desc>Show PagerDuty status.</xbar.desc>

SCRIPT_DIR=$(dirname -- $(realpath "${BASH_SOURCE[0]}"))
exec "${SCRIPT_DIR}/xbar_pagerduty.sh"
