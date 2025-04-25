pagerduty-xbar(1) - don't miss your PagerDuty shift again
============================

## SYNOPSIS
`pagerduty-xbar` - Get PagerDuty incoming shifts in a format suitable for [xbar][xbar] or [SwiftBar][swiftbar].

## DESCRIPTION

Show Pager duty status in [xbar][xbar].

[xbar]: https://github.com/matryer/xbar
[swiftbar]: https://github.com/swiftbar/SwiftBar

## CONFIGURATION

Requires the following environment variables to work (in `~/.config/pg_xbar.conf`)

- `PG_SCHEDULE_ID` - ID of the schedule, can be take from PagerDuty URL.
- `PG_AUTH_TOKEN` - Authentication token, generate it in PagerDuty settings.
- `PG_USER_ID` - your user ID, available in your profile URL.
- `PG_ICONS` - Optional set of there icons: on-duty, tomorrow-on-duty, not-on-duty.
- `PG_DOMAIN` - An optional PagerDuty domain that will be used in the URLS.

Link the `xbar_pagerduty_plugin.sh` script to xbar or SwiftBar:

```
ln -s $(realpath /opt/homebrew/bin/xbar_pagerduty_plugin.sh) "$HOME/Library/Application Support/xbar/plugins/pagerduty.5m.sh"
# or
ln -s $(realpath /opt/homebrew/bin/xbar_pagerduty_plugin.sh) "$HOME/Library/Application Support/SwiftBar/plugins/pagerduty.5m.sh"
```
