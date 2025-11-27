// Automatically generated rust module for 'spotware.95.proto' file

#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use quacky::{
    sizeof::{sizeof_len, sizeof_varint},
    BytesReader, MessageRead, MessageWrite, Result, Writer, WriterBackend,
};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoPayloadType {
    PROTO_MESSAGE = 5,
    ERROR_RES = 50,
    HEARTBEAT_EVENT = 51,
}

impl Default for ProtoPayloadType {
    fn default() -> Self {
        ProtoPayloadType::PROTO_MESSAGE
    }
}

impl From<i32> for ProtoPayloadType {
    fn from(i: i32) -> Self {
        match i {
            5 => Self::PROTO_MESSAGE,
            50 => Self::ERROR_RES,
            51 => Self::HEARTBEAT_EVENT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoPayloadType {
    fn from(s: &'a str) -> Self {
        match s {
            "PROTO_MESSAGE" => Self::PROTO_MESSAGE,
            "ERROR_RES" => Self::ERROR_RES,
            "HEARTBEAT_EVENT" => Self::HEARTBEAT_EVENT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoErrorCode {
    UNKNOWN_ERROR = 1,
    UNSUPPORTED_MESSAGE = 2,
    INVALID_REQUEST = 3,
    TIMEOUT_ERROR = 5,
    ENTITY_NOT_FOUND = 6,
    CANT_ROUTE_REQUEST = 7,
    FRAME_TOO_LONG = 8,
    MARKET_CLOSED = 9,
    CONCURRENT_MODIFICATION = 10,
    BLOCKED_PAYLOAD_TYPE = 11,
}

impl Default for ProtoErrorCode {
    fn default() -> Self {
        ProtoErrorCode::UNKNOWN_ERROR
    }
}

impl From<i32> for ProtoErrorCode {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::UNKNOWN_ERROR,
            2 => Self::UNSUPPORTED_MESSAGE,
            3 => Self::INVALID_REQUEST,
            5 => Self::TIMEOUT_ERROR,
            6 => Self::ENTITY_NOT_FOUND,
            7 => Self::CANT_ROUTE_REQUEST,
            8 => Self::FRAME_TOO_LONG,
            9 => Self::MARKET_CLOSED,
            10 => Self::CONCURRENT_MODIFICATION,
            11 => Self::BLOCKED_PAYLOAD_TYPE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoErrorCode {
    fn from(s: &'a str) -> Self {
        match s {
            "UNKNOWN_ERROR" => Self::UNKNOWN_ERROR,
            "UNSUPPORTED_MESSAGE" => Self::UNSUPPORTED_MESSAGE,
            "INVALID_REQUEST" => Self::INVALID_REQUEST,
            "TIMEOUT_ERROR" => Self::TIMEOUT_ERROR,
            "ENTITY_NOT_FOUND" => Self::ENTITY_NOT_FOUND,
            "CANT_ROUTE_REQUEST" => Self::CANT_ROUTE_REQUEST,
            "FRAME_TOO_LONG" => Self::FRAME_TOO_LONG,
            "MARKET_CLOSED" => Self::MARKET_CLOSED,
            "CONCURRENT_MODIFICATION" => Self::CONCURRENT_MODIFICATION,
            "BLOCKED_PAYLOAD_TYPE" => Self::BLOCKED_PAYLOAD_TYPE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAPayloadType {
    PROTO_OA_APPLICATION_AUTH_REQ = 2100,
    PROTO_OA_APPLICATION_AUTH_RES = 2101,
    PROTO_OA_ACCOUNT_AUTH_REQ = 2102,
    PROTO_OA_ACCOUNT_AUTH_RES = 2103,
    PROTO_OA_VERSION_REQ = 2104,
    PROTO_OA_VERSION_RES = 2105,
    PROTO_OA_NEW_ORDER_REQ = 2106,
    PROTO_OA_TRAILING_SL_CHANGED_EVENT = 2107,
    PROTO_OA_CANCEL_ORDER_REQ = 2108,
    PROTO_OA_AMEND_ORDER_REQ = 2109,
    PROTO_OA_AMEND_POSITION_SLTP_REQ = 2110,
    PROTO_OA_CLOSE_POSITION_REQ = 2111,
    PROTO_OA_ASSET_LIST_REQ = 2112,
    PROTO_OA_ASSET_LIST_RES = 2113,
    PROTO_OA_SYMBOLS_LIST_REQ = 2114,
    PROTO_OA_SYMBOLS_LIST_RES = 2115,
    PROTO_OA_SYMBOL_BY_ID_REQ = 2116,
    PROTO_OA_SYMBOL_BY_ID_RES = 2117,
    PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ = 2118,
    PROTO_OA_SYMBOLS_FOR_CONVERSION_RES = 2119,
    PROTO_OA_SYMBOL_CHANGED_EVENT = 2120,
    PROTO_OA_TRADER_REQ = 2121,
    PROTO_OA_TRADER_RES = 2122,
    PROTO_OA_TRADER_UPDATE_EVENT = 2123,
    PROTO_OA_RECONCILE_REQ = 2124,
    PROTO_OA_RECONCILE_RES = 2125,
    PROTO_OA_EXECUTION_EVENT = 2126,
    PROTO_OA_SUBSCRIBE_SPOTS_REQ = 2127,
    PROTO_OA_SUBSCRIBE_SPOTS_RES = 2128,
    PROTO_OA_UNSUBSCRIBE_SPOTS_REQ = 2129,
    PROTO_OA_UNSUBSCRIBE_SPOTS_RES = 2130,
    PROTO_OA_SPOT_EVENT = 2131,
    PROTO_OA_ORDER_ERROR_EVENT = 2132,
    PROTO_OA_DEAL_LIST_REQ = 2133,
    PROTO_OA_DEAL_LIST_RES = 2134,
    PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ = 2135,
    PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ = 2136,
    PROTO_OA_GET_TRENDBARS_REQ = 2137,
    PROTO_OA_GET_TRENDBARS_RES = 2138,
    PROTO_OA_EXPECTED_MARGIN_REQ = 2139,
    PROTO_OA_EXPECTED_MARGIN_RES = 2140,
    PROTO_OA_MARGIN_CHANGED_EVENT = 2141,
    PROTO_OA_ERROR_RES = 2142,
    PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ = 2143,
    PROTO_OA_CASH_FLOW_HISTORY_LIST_RES = 2144,
    PROTO_OA_GET_TICKDATA_REQ = 2145,
    PROTO_OA_GET_TICKDATA_RES = 2146,
    PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT = 2147,
    PROTO_OA_CLIENT_DISCONNECT_EVENT = 2148,
    PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ = 2149,
    PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES = 2150,
    PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ = 2151,
    PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES = 2152,
    PROTO_OA_ASSET_CLASS_LIST_REQ = 2153,
    PROTO_OA_ASSET_CLASS_LIST_RES = 2154,
    PROTO_OA_DEPTH_EVENT = 2155,
    PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ = 2156,
    PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES = 2157,
    PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ = 2158,
    PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES = 2159,
    PROTO_OA_SYMBOL_CATEGORY_REQ = 2160,
    PROTO_OA_SYMBOL_CATEGORY_RES = 2161,
    PROTO_OA_ACCOUNT_LOGOUT_REQ = 2162,
    PROTO_OA_ACCOUNT_LOGOUT_RES = 2163,
    PROTO_OA_ACCOUNT_DISCONNECT_EVENT = 2164,
    PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES = 2165,
    PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES = 2166,
    PROTO_OA_MARGIN_CALL_LIST_REQ = 2167,
    PROTO_OA_MARGIN_CALL_LIST_RES = 2168,
    PROTO_OA_MARGIN_CALL_UPDATE_REQ = 2169,
    PROTO_OA_MARGIN_CALL_UPDATE_RES = 2170,
    PROTO_OA_MARGIN_CALL_UPDATE_EVENT = 2171,
    PROTO_OA_MARGIN_CALL_TRIGGER_EVENT = 2172,
    PROTO_OA_REFRESH_TOKEN_REQ = 2173,
    PROTO_OA_REFRESH_TOKEN_RES = 2174,
    PROTO_OA_ORDER_LIST_REQ = 2175,
    PROTO_OA_ORDER_LIST_RES = 2176,
    PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ = 2177,
    PROTO_OA_GET_DYNAMIC_LEVERAGE_RES = 2178,
    PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ = 2179,
    PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES = 2180,
    PROTO_OA_ORDER_DETAILS_REQ = 2181,
    PROTO_OA_ORDER_DETAILS_RES = 2182,
    PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ = 2183,
    PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES = 2184,
    PROTO_OA_DEAL_OFFSET_LIST_REQ = 2185,
    PROTO_OA_DEAL_OFFSET_LIST_RES = 2186,
    PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ = 2187,
    PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES = 2188,
}

impl Default for ProtoOAPayloadType {
    fn default() -> Self {
        ProtoOAPayloadType::PROTO_OA_APPLICATION_AUTH_REQ
    }
}

impl From<i32> for ProtoOAPayloadType {
    fn from(i: i32) -> Self {
        match i {
            2100 => Self::PROTO_OA_APPLICATION_AUTH_REQ,
            2101 => Self::PROTO_OA_APPLICATION_AUTH_RES,
            2102 => Self::PROTO_OA_ACCOUNT_AUTH_REQ,
            2103 => Self::PROTO_OA_ACCOUNT_AUTH_RES,
            2104 => Self::PROTO_OA_VERSION_REQ,
            2105 => Self::PROTO_OA_VERSION_RES,
            2106 => Self::PROTO_OA_NEW_ORDER_REQ,
            2107 => Self::PROTO_OA_TRAILING_SL_CHANGED_EVENT,
            2108 => Self::PROTO_OA_CANCEL_ORDER_REQ,
            2109 => Self::PROTO_OA_AMEND_ORDER_REQ,
            2110 => Self::PROTO_OA_AMEND_POSITION_SLTP_REQ,
            2111 => Self::PROTO_OA_CLOSE_POSITION_REQ,
            2112 => Self::PROTO_OA_ASSET_LIST_REQ,
            2113 => Self::PROTO_OA_ASSET_LIST_RES,
            2114 => Self::PROTO_OA_SYMBOLS_LIST_REQ,
            2115 => Self::PROTO_OA_SYMBOLS_LIST_RES,
            2116 => Self::PROTO_OA_SYMBOL_BY_ID_REQ,
            2117 => Self::PROTO_OA_SYMBOL_BY_ID_RES,
            2118 => Self::PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ,
            2119 => Self::PROTO_OA_SYMBOLS_FOR_CONVERSION_RES,
            2120 => Self::PROTO_OA_SYMBOL_CHANGED_EVENT,
            2121 => Self::PROTO_OA_TRADER_REQ,
            2122 => Self::PROTO_OA_TRADER_RES,
            2123 => Self::PROTO_OA_TRADER_UPDATE_EVENT,
            2124 => Self::PROTO_OA_RECONCILE_REQ,
            2125 => Self::PROTO_OA_RECONCILE_RES,
            2126 => Self::PROTO_OA_EXECUTION_EVENT,
            2127 => Self::PROTO_OA_SUBSCRIBE_SPOTS_REQ,
            2128 => Self::PROTO_OA_SUBSCRIBE_SPOTS_RES,
            2129 => Self::PROTO_OA_UNSUBSCRIBE_SPOTS_REQ,
            2130 => Self::PROTO_OA_UNSUBSCRIBE_SPOTS_RES,
            2131 => Self::PROTO_OA_SPOT_EVENT,
            2132 => Self::PROTO_OA_ORDER_ERROR_EVENT,
            2133 => Self::PROTO_OA_DEAL_LIST_REQ,
            2134 => Self::PROTO_OA_DEAL_LIST_RES,
            2135 => Self::PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ,
            2136 => Self::PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ,
            2137 => Self::PROTO_OA_GET_TRENDBARS_REQ,
            2138 => Self::PROTO_OA_GET_TRENDBARS_RES,
            2139 => Self::PROTO_OA_EXPECTED_MARGIN_REQ,
            2140 => Self::PROTO_OA_EXPECTED_MARGIN_RES,
            2141 => Self::PROTO_OA_MARGIN_CHANGED_EVENT,
            2142 => Self::PROTO_OA_ERROR_RES,
            2143 => Self::PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ,
            2144 => Self::PROTO_OA_CASH_FLOW_HISTORY_LIST_RES,
            2145 => Self::PROTO_OA_GET_TICKDATA_REQ,
            2146 => Self::PROTO_OA_GET_TICKDATA_RES,
            2147 => Self::PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT,
            2148 => Self::PROTO_OA_CLIENT_DISCONNECT_EVENT,
            2149 => Self::PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ,
            2150 => Self::PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES,
            2151 => Self::PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ,
            2152 => Self::PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES,
            2153 => Self::PROTO_OA_ASSET_CLASS_LIST_REQ,
            2154 => Self::PROTO_OA_ASSET_CLASS_LIST_RES,
            2155 => Self::PROTO_OA_DEPTH_EVENT,
            2156 => Self::PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ,
            2157 => Self::PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES,
            2158 => Self::PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ,
            2159 => Self::PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES,
            2160 => Self::PROTO_OA_SYMBOL_CATEGORY_REQ,
            2161 => Self::PROTO_OA_SYMBOL_CATEGORY_RES,
            2162 => Self::PROTO_OA_ACCOUNT_LOGOUT_REQ,
            2163 => Self::PROTO_OA_ACCOUNT_LOGOUT_RES,
            2164 => Self::PROTO_OA_ACCOUNT_DISCONNECT_EVENT,
            2165 => Self::PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES,
            2166 => Self::PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES,
            2167 => Self::PROTO_OA_MARGIN_CALL_LIST_REQ,
            2168 => Self::PROTO_OA_MARGIN_CALL_LIST_RES,
            2169 => Self::PROTO_OA_MARGIN_CALL_UPDATE_REQ,
            2170 => Self::PROTO_OA_MARGIN_CALL_UPDATE_RES,
            2171 => Self::PROTO_OA_MARGIN_CALL_UPDATE_EVENT,
            2172 => Self::PROTO_OA_MARGIN_CALL_TRIGGER_EVENT,
            2173 => Self::PROTO_OA_REFRESH_TOKEN_REQ,
            2174 => Self::PROTO_OA_REFRESH_TOKEN_RES,
            2175 => Self::PROTO_OA_ORDER_LIST_REQ,
            2176 => Self::PROTO_OA_ORDER_LIST_RES,
            2177 => Self::PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ,
            2178 => Self::PROTO_OA_GET_DYNAMIC_LEVERAGE_RES,
            2179 => Self::PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ,
            2180 => Self::PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES,
            2181 => Self::PROTO_OA_ORDER_DETAILS_REQ,
            2182 => Self::PROTO_OA_ORDER_DETAILS_RES,
            2183 => Self::PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ,
            2184 => Self::PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES,
            2185 => Self::PROTO_OA_DEAL_OFFSET_LIST_REQ,
            2186 => Self::PROTO_OA_DEAL_OFFSET_LIST_RES,
            2187 => Self::PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ,
            2188 => Self::PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAPayloadType {
    fn from(s: &'a str) -> Self {
        match s {
            "PROTO_OA_APPLICATION_AUTH_REQ" => Self::PROTO_OA_APPLICATION_AUTH_REQ,
            "PROTO_OA_APPLICATION_AUTH_RES" => Self::PROTO_OA_APPLICATION_AUTH_RES,
            "PROTO_OA_ACCOUNT_AUTH_REQ" => Self::PROTO_OA_ACCOUNT_AUTH_REQ,
            "PROTO_OA_ACCOUNT_AUTH_RES" => Self::PROTO_OA_ACCOUNT_AUTH_RES,
            "PROTO_OA_VERSION_REQ" => Self::PROTO_OA_VERSION_REQ,
            "PROTO_OA_VERSION_RES" => Self::PROTO_OA_VERSION_RES,
            "PROTO_OA_NEW_ORDER_REQ" => Self::PROTO_OA_NEW_ORDER_REQ,
            "PROTO_OA_TRAILING_SL_CHANGED_EVENT" => Self::PROTO_OA_TRAILING_SL_CHANGED_EVENT,
            "PROTO_OA_CANCEL_ORDER_REQ" => Self::PROTO_OA_CANCEL_ORDER_REQ,
            "PROTO_OA_AMEND_ORDER_REQ" => Self::PROTO_OA_AMEND_ORDER_REQ,
            "PROTO_OA_AMEND_POSITION_SLTP_REQ" => Self::PROTO_OA_AMEND_POSITION_SLTP_REQ,
            "PROTO_OA_CLOSE_POSITION_REQ" => Self::PROTO_OA_CLOSE_POSITION_REQ,
            "PROTO_OA_ASSET_LIST_REQ" => Self::PROTO_OA_ASSET_LIST_REQ,
            "PROTO_OA_ASSET_LIST_RES" => Self::PROTO_OA_ASSET_LIST_RES,
            "PROTO_OA_SYMBOLS_LIST_REQ" => Self::PROTO_OA_SYMBOLS_LIST_REQ,
            "PROTO_OA_SYMBOLS_LIST_RES" => Self::PROTO_OA_SYMBOLS_LIST_RES,
            "PROTO_OA_SYMBOL_BY_ID_REQ" => Self::PROTO_OA_SYMBOL_BY_ID_REQ,
            "PROTO_OA_SYMBOL_BY_ID_RES" => Self::PROTO_OA_SYMBOL_BY_ID_RES,
            "PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ" => Self::PROTO_OA_SYMBOLS_FOR_CONVERSION_REQ,
            "PROTO_OA_SYMBOLS_FOR_CONVERSION_RES" => Self::PROTO_OA_SYMBOLS_FOR_CONVERSION_RES,
            "PROTO_OA_SYMBOL_CHANGED_EVENT" => Self::PROTO_OA_SYMBOL_CHANGED_EVENT,
            "PROTO_OA_TRADER_REQ" => Self::PROTO_OA_TRADER_REQ,
            "PROTO_OA_TRADER_RES" => Self::PROTO_OA_TRADER_RES,
            "PROTO_OA_TRADER_UPDATE_EVENT" => Self::PROTO_OA_TRADER_UPDATE_EVENT,
            "PROTO_OA_RECONCILE_REQ" => Self::PROTO_OA_RECONCILE_REQ,
            "PROTO_OA_RECONCILE_RES" => Self::PROTO_OA_RECONCILE_RES,
            "PROTO_OA_EXECUTION_EVENT" => Self::PROTO_OA_EXECUTION_EVENT,
            "PROTO_OA_SUBSCRIBE_SPOTS_REQ" => Self::PROTO_OA_SUBSCRIBE_SPOTS_REQ,
            "PROTO_OA_SUBSCRIBE_SPOTS_RES" => Self::PROTO_OA_SUBSCRIBE_SPOTS_RES,
            "PROTO_OA_UNSUBSCRIBE_SPOTS_REQ" => Self::PROTO_OA_UNSUBSCRIBE_SPOTS_REQ,
            "PROTO_OA_UNSUBSCRIBE_SPOTS_RES" => Self::PROTO_OA_UNSUBSCRIBE_SPOTS_RES,
            "PROTO_OA_SPOT_EVENT" => Self::PROTO_OA_SPOT_EVENT,
            "PROTO_OA_ORDER_ERROR_EVENT" => Self::PROTO_OA_ORDER_ERROR_EVENT,
            "PROTO_OA_DEAL_LIST_REQ" => Self::PROTO_OA_DEAL_LIST_REQ,
            "PROTO_OA_DEAL_LIST_RES" => Self::PROTO_OA_DEAL_LIST_RES,
            "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ" => Self::PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_REQ,
            "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ" => Self::PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_REQ,
            "PROTO_OA_GET_TRENDBARS_REQ" => Self::PROTO_OA_GET_TRENDBARS_REQ,
            "PROTO_OA_GET_TRENDBARS_RES" => Self::PROTO_OA_GET_TRENDBARS_RES,
            "PROTO_OA_EXPECTED_MARGIN_REQ" => Self::PROTO_OA_EXPECTED_MARGIN_REQ,
            "PROTO_OA_EXPECTED_MARGIN_RES" => Self::PROTO_OA_EXPECTED_MARGIN_RES,
            "PROTO_OA_MARGIN_CHANGED_EVENT" => Self::PROTO_OA_MARGIN_CHANGED_EVENT,
            "PROTO_OA_ERROR_RES" => Self::PROTO_OA_ERROR_RES,
            "PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ" => Self::PROTO_OA_CASH_FLOW_HISTORY_LIST_REQ,
            "PROTO_OA_CASH_FLOW_HISTORY_LIST_RES" => Self::PROTO_OA_CASH_FLOW_HISTORY_LIST_RES,
            "PROTO_OA_GET_TICKDATA_REQ" => Self::PROTO_OA_GET_TICKDATA_REQ,
            "PROTO_OA_GET_TICKDATA_RES" => Self::PROTO_OA_GET_TICKDATA_RES,
            "PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT" => Self::PROTO_OA_ACCOUNTS_TOKEN_INVALIDATED_EVENT,
            "PROTO_OA_CLIENT_DISCONNECT_EVENT" => Self::PROTO_OA_CLIENT_DISCONNECT_EVENT,
            "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ" => Self::PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_REQ,
            "PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES" => Self::PROTO_OA_GET_ACCOUNTS_BY_ACCESS_TOKEN_RES,
            "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ" => Self::PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_REQ,
            "PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES" => Self::PROTO_OA_GET_CTID_PROFILE_BY_TOKEN_RES,
            "PROTO_OA_ASSET_CLASS_LIST_REQ" => Self::PROTO_OA_ASSET_CLASS_LIST_REQ,
            "PROTO_OA_ASSET_CLASS_LIST_RES" => Self::PROTO_OA_ASSET_CLASS_LIST_RES,
            "PROTO_OA_DEPTH_EVENT" => Self::PROTO_OA_DEPTH_EVENT,
            "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ" => Self::PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_REQ,
            "PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES" => Self::PROTO_OA_SUBSCRIBE_DEPTH_QUOTES_RES,
            "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ" => Self::PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_REQ,
            "PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES" => Self::PROTO_OA_UNSUBSCRIBE_DEPTH_QUOTES_RES,
            "PROTO_OA_SYMBOL_CATEGORY_REQ" => Self::PROTO_OA_SYMBOL_CATEGORY_REQ,
            "PROTO_OA_SYMBOL_CATEGORY_RES" => Self::PROTO_OA_SYMBOL_CATEGORY_RES,
            "PROTO_OA_ACCOUNT_LOGOUT_REQ" => Self::PROTO_OA_ACCOUNT_LOGOUT_REQ,
            "PROTO_OA_ACCOUNT_LOGOUT_RES" => Self::PROTO_OA_ACCOUNT_LOGOUT_RES,
            "PROTO_OA_ACCOUNT_DISCONNECT_EVENT" => Self::PROTO_OA_ACCOUNT_DISCONNECT_EVENT,
            "PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES" => Self::PROTO_OA_SUBSCRIBE_LIVE_TRENDBAR_RES,
            "PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES" => Self::PROTO_OA_UNSUBSCRIBE_LIVE_TRENDBAR_RES,
            "PROTO_OA_MARGIN_CALL_LIST_REQ" => Self::PROTO_OA_MARGIN_CALL_LIST_REQ,
            "PROTO_OA_MARGIN_CALL_LIST_RES" => Self::PROTO_OA_MARGIN_CALL_LIST_RES,
            "PROTO_OA_MARGIN_CALL_UPDATE_REQ" => Self::PROTO_OA_MARGIN_CALL_UPDATE_REQ,
            "PROTO_OA_MARGIN_CALL_UPDATE_RES" => Self::PROTO_OA_MARGIN_CALL_UPDATE_RES,
            "PROTO_OA_MARGIN_CALL_UPDATE_EVENT" => Self::PROTO_OA_MARGIN_CALL_UPDATE_EVENT,
            "PROTO_OA_MARGIN_CALL_TRIGGER_EVENT" => Self::PROTO_OA_MARGIN_CALL_TRIGGER_EVENT,
            "PROTO_OA_REFRESH_TOKEN_REQ" => Self::PROTO_OA_REFRESH_TOKEN_REQ,
            "PROTO_OA_REFRESH_TOKEN_RES" => Self::PROTO_OA_REFRESH_TOKEN_RES,
            "PROTO_OA_ORDER_LIST_REQ" => Self::PROTO_OA_ORDER_LIST_REQ,
            "PROTO_OA_ORDER_LIST_RES" => Self::PROTO_OA_ORDER_LIST_RES,
            "PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ" => Self::PROTO_OA_GET_DYNAMIC_LEVERAGE_REQ,
            "PROTO_OA_GET_DYNAMIC_LEVERAGE_RES" => Self::PROTO_OA_GET_DYNAMIC_LEVERAGE_RES,
            "PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ" => Self::PROTO_OA_DEAL_LIST_BY_POSITION_ID_REQ,
            "PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES" => Self::PROTO_OA_DEAL_LIST_BY_POSITION_ID_RES,
            "PROTO_OA_ORDER_DETAILS_REQ" => Self::PROTO_OA_ORDER_DETAILS_REQ,
            "PROTO_OA_ORDER_DETAILS_RES" => Self::PROTO_OA_ORDER_DETAILS_RES,
            "PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ" => Self::PROTO_OA_ORDER_LIST_BY_POSITION_ID_REQ,
            "PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES" => Self::PROTO_OA_ORDER_LIST_BY_POSITION_ID_RES,
            "PROTO_OA_DEAL_OFFSET_LIST_REQ" => Self::PROTO_OA_DEAL_OFFSET_LIST_REQ,
            "PROTO_OA_DEAL_OFFSET_LIST_RES" => Self::PROTO_OA_DEAL_OFFSET_LIST_RES,
            "PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ" => Self::PROTO_OA_GET_POSITION_UNREALIZED_PNL_REQ,
            "PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES" => Self::PROTO_OA_GET_POSITION_UNREALIZED_PNL_RES,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOADayOfWeek {
    NONE = 0,
    MONDAY = 1,
    TUESDAY = 2,
    WEDNESDAY = 3,
    THURSDAY = 4,
    FRIDAY = 5,
    SATURDAY = 6,
    SUNDAY = 7,
}

impl Default for ProtoOADayOfWeek {
    fn default() -> Self {
        ProtoOADayOfWeek::NONE
    }
}

impl From<i32> for ProtoOADayOfWeek {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::NONE,
            1 => Self::MONDAY,
            2 => Self::TUESDAY,
            3 => Self::WEDNESDAY,
            4 => Self::THURSDAY,
            5 => Self::FRIDAY,
            6 => Self::SATURDAY,
            7 => Self::SUNDAY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOADayOfWeek {
    fn from(s: &'a str) -> Self {
        match s {
            "NONE" => Self::NONE,
            "MONDAY" => Self::MONDAY,
            "TUESDAY" => Self::TUESDAY,
            "WEDNESDAY" => Self::WEDNESDAY,
            "THURSDAY" => Self::THURSDAY,
            "FRIDAY" => Self::FRIDAY,
            "SATURDAY" => Self::SATURDAY,
            "SUNDAY" => Self::SUNDAY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOACommissionType {
    USD_PER_MILLION_USD = 1,
    USD_PER_LOT = 2,
    PERCENTAGE_OF_VALUE = 3,
    QUOTE_CCY_PER_LOT = 4,
}

impl Default for ProtoOACommissionType {
    fn default() -> Self {
        ProtoOACommissionType::USD_PER_MILLION_USD
    }
}

impl From<i32> for ProtoOACommissionType {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::USD_PER_MILLION_USD,
            2 => Self::USD_PER_LOT,
            3 => Self::PERCENTAGE_OF_VALUE,
            4 => Self::QUOTE_CCY_PER_LOT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOACommissionType {
    fn from(s: &'a str) -> Self {
        match s {
            "USD_PER_MILLION_USD" => Self::USD_PER_MILLION_USD,
            "USD_PER_LOT" => Self::USD_PER_LOT,
            "PERCENTAGE_OF_VALUE" => Self::PERCENTAGE_OF_VALUE,
            "QUOTE_CCY_PER_LOT" => Self::QUOTE_CCY_PER_LOT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOASymbolDistanceType {
    SYMBOL_DISTANCE_IN_POINTS = 1,
    SYMBOL_DISTANCE_IN_PERCENTAGE = 2,
}

impl Default for ProtoOASymbolDistanceType {
    fn default() -> Self {
        ProtoOASymbolDistanceType::SYMBOL_DISTANCE_IN_POINTS
    }
}

impl From<i32> for ProtoOASymbolDistanceType {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::SYMBOL_DISTANCE_IN_POINTS,
            2 => Self::SYMBOL_DISTANCE_IN_PERCENTAGE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOASymbolDistanceType {
    fn from(s: &'a str) -> Self {
        match s {
            "SYMBOL_DISTANCE_IN_POINTS" => Self::SYMBOL_DISTANCE_IN_POINTS,
            "SYMBOL_DISTANCE_IN_PERCENTAGE" => Self::SYMBOL_DISTANCE_IN_PERCENTAGE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAMinCommissionType {
    CURRENCY = 1,
    QUOTE_CURRENCY = 2,
}

impl Default for ProtoOAMinCommissionType {
    fn default() -> Self {
        ProtoOAMinCommissionType::CURRENCY
    }
}

impl From<i32> for ProtoOAMinCommissionType {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::CURRENCY,
            2 => Self::QUOTE_CURRENCY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAMinCommissionType {
    fn from(s: &'a str) -> Self {
        match s {
            "CURRENCY" => Self::CURRENCY,
            "QUOTE_CURRENCY" => Self::QUOTE_CURRENCY,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOATradingMode {
    ENABLED = 0,
    DISABLED_WITHOUT_PENDINGS_EXECUTION = 1,
    DISABLED_WITH_PENDINGS_EXECUTION = 2,
    CLOSE_ONLY_MODE = 3,
}

impl Default for ProtoOATradingMode {
    fn default() -> Self {
        ProtoOATradingMode::ENABLED
    }
}

impl From<i32> for ProtoOATradingMode {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::ENABLED,
            1 => Self::DISABLED_WITHOUT_PENDINGS_EXECUTION,
            2 => Self::DISABLED_WITH_PENDINGS_EXECUTION,
            3 => Self::CLOSE_ONLY_MODE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOATradingMode {
    fn from(s: &'a str) -> Self {
        match s {
            "ENABLED" => Self::ENABLED,
            "DISABLED_WITHOUT_PENDINGS_EXECUTION" => Self::DISABLED_WITHOUT_PENDINGS_EXECUTION,
            "DISABLED_WITH_PENDINGS_EXECUTION" => Self::DISABLED_WITH_PENDINGS_EXECUTION,
            "CLOSE_ONLY_MODE" => Self::CLOSE_ONLY_MODE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOASwapCalculationType {
    PIPS = 0,
    PERCENTAGE = 1,
    POINTS = 2,
}

impl Default for ProtoOASwapCalculationType {
    fn default() -> Self {
        ProtoOASwapCalculationType::PIPS
    }
}

impl From<i32> for ProtoOASwapCalculationType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::PIPS,
            1 => Self::PERCENTAGE,
            2 => Self::POINTS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOASwapCalculationType {
    fn from(s: &'a str) -> Self {
        match s {
            "PIPS" => Self::PIPS,
            "PERCENTAGE" => Self::PERCENTAGE,
            "POINTS" => Self::POINTS,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAAccessRights {
    FULL_ACCESS = 0,
    CLOSE_ONLY = 1,
    NO_TRADING = 2,
    NO_LOGIN = 3,
}

impl Default for ProtoOAAccessRights {
    fn default() -> Self {
        ProtoOAAccessRights::FULL_ACCESS
    }
}

impl From<i32> for ProtoOAAccessRights {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::FULL_ACCESS,
            1 => Self::CLOSE_ONLY,
            2 => Self::NO_TRADING,
            3 => Self::NO_LOGIN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAAccessRights {
    fn from(s: &'a str) -> Self {
        match s {
            "FULL_ACCESS" => Self::FULL_ACCESS,
            "CLOSE_ONLY" => Self::CLOSE_ONLY,
            "NO_TRADING" => Self::NO_TRADING,
            "NO_LOGIN" => Self::NO_LOGIN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOATotalMarginCalculationType {
    MAX = 0,
    SUM = 1,
    NET = 2,
}

impl Default for ProtoOATotalMarginCalculationType {
    fn default() -> Self {
        ProtoOATotalMarginCalculationType::MAX
    }
}

impl From<i32> for ProtoOATotalMarginCalculationType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::MAX,
            1 => Self::SUM,
            2 => Self::NET,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOATotalMarginCalculationType {
    fn from(s: &'a str) -> Self {
        match s {
            "MAX" => Self::MAX,
            "SUM" => Self::SUM,
            "NET" => Self::NET,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAAccountType {
    HEDGED = 0,
    NETTED = 1,
    SPREAD_BETTING = 2,
}

impl Default for ProtoOAAccountType {
    fn default() -> Self {
        ProtoOAAccountType::HEDGED
    }
}

impl From<i32> for ProtoOAAccountType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::HEDGED,
            1 => Self::NETTED,
            2 => Self::SPREAD_BETTING,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAAccountType {
    fn from(s: &'a str) -> Self {
        match s {
            "HEDGED" => Self::HEDGED,
            "NETTED" => Self::NETTED,
            "SPREAD_BETTING" => Self::SPREAD_BETTING,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOANotificationType {
    MARGIN_LEVEL_THRESHOLD_1 = 61,
    MARGIN_LEVEL_THRESHOLD_2 = 62,
    MARGIN_LEVEL_THRESHOLD_3 = 63,
}

impl Default for ProtoOANotificationType {
    fn default() -> Self {
        ProtoOANotificationType::MARGIN_LEVEL_THRESHOLD_1
    }
}

impl From<i32> for ProtoOANotificationType {
    fn from(i: i32) -> Self {
        match i {
            61 => Self::MARGIN_LEVEL_THRESHOLD_1,
            62 => Self::MARGIN_LEVEL_THRESHOLD_2,
            63 => Self::MARGIN_LEVEL_THRESHOLD_3,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOANotificationType {
    fn from(s: &'a str) -> Self {
        match s {
            "MARGIN_LEVEL_THRESHOLD_1" => Self::MARGIN_LEVEL_THRESHOLD_1,
            "MARGIN_LEVEL_THRESHOLD_2" => Self::MARGIN_LEVEL_THRESHOLD_2,
            "MARGIN_LEVEL_THRESHOLD_3" => Self::MARGIN_LEVEL_THRESHOLD_3,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAErrorCode {
    OA_AUTH_TOKEN_EXPIRED = 1,
    ACCOUNT_NOT_AUTHORIZED = 2,
    RET_NO_SUCH_LOGIN = 12,
    ALREADY_LOGGED_IN = 14,
    INCORRECT_BOUNDARIES = 35,
    RET_ACCOUNT_DISABLED = 64,
    CONNECTIONS_LIMIT_EXCEEDED = 67,
    WORSE_GSL_NOT_ALLOWED = 68,
    SYMBOL_HAS_HOLIDAY = 69,
    CH_CLIENT_AUTH_FAILURE = 101,
    CH_CLIENT_NOT_AUTHENTICATED = 102,
    CH_CLIENT_ALREADY_AUTHENTICATED = 103,
    CH_ACCESS_TOKEN_INVALID = 104,
    CH_SERVER_NOT_REACHABLE = 105,
    CH_CTID_TRADER_ACCOUNT_NOT_FOUND = 106,
    CH_OA_CLIENT_NOT_FOUND = 107,
    REQUEST_FREQUENCY_EXCEEDED = 108,
    SERVER_IS_UNDER_MAINTENANCE = 109,
    CHANNEL_IS_BLOCKED = 110,
    NOT_SUBSCRIBED_TO_SPOTS = 112,
    ALREADY_SUBSCRIBED = 113,
    SYMBOL_NOT_FOUND = 114,
    UNKNOWN_SYMBOL = 115,
    NO_QUOTES = 117,
    NOT_ENOUGH_MONEY = 118,
    MAX_EXPOSURE_REACHED = 119,
    POSITION_NOT_FOUND = 120,
    ORDER_NOT_FOUND = 121,
    POSITION_NOT_OPEN = 122,
    POSITION_LOCKED = 123,
    TOO_MANY_POSITIONS = 124,
    TRADING_BAD_VOLUME = 125,
    TRADING_BAD_STOPS = 126,
    TRADING_BAD_PRICES = 127,
    TRADING_BAD_STAKE = 128,
    PROTECTION_IS_TOO_CLOSE_TO_MARKET = 129,
    TRADING_BAD_EXPIRATION_DATE = 130,
    PENDING_EXECUTION = 131,
    TRADING_DISABLED = 132,
    TRADING_NOT_ALLOWED = 133,
    UNABLE_TO_CANCEL_ORDER = 134,
    UNABLE_TO_AMEND_ORDER = 135,
    SHORT_SELLING_NOT_ALLOWED = 136,
}

impl Default for ProtoOAErrorCode {
    fn default() -> Self {
        ProtoOAErrorCode::OA_AUTH_TOKEN_EXPIRED
    }
}

impl From<i32> for ProtoOAErrorCode {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::OA_AUTH_TOKEN_EXPIRED,
            2 => Self::ACCOUNT_NOT_AUTHORIZED,
            12 => Self::RET_NO_SUCH_LOGIN,
            14 => Self::ALREADY_LOGGED_IN,
            35 => Self::INCORRECT_BOUNDARIES,
            64 => Self::RET_ACCOUNT_DISABLED,
            67 => Self::CONNECTIONS_LIMIT_EXCEEDED,
            68 => Self::WORSE_GSL_NOT_ALLOWED,
            69 => Self::SYMBOL_HAS_HOLIDAY,
            101 => Self::CH_CLIENT_AUTH_FAILURE,
            102 => Self::CH_CLIENT_NOT_AUTHENTICATED,
            103 => Self::CH_CLIENT_ALREADY_AUTHENTICATED,
            104 => Self::CH_ACCESS_TOKEN_INVALID,
            105 => Self::CH_SERVER_NOT_REACHABLE,
            106 => Self::CH_CTID_TRADER_ACCOUNT_NOT_FOUND,
            107 => Self::CH_OA_CLIENT_NOT_FOUND,
            108 => Self::REQUEST_FREQUENCY_EXCEEDED,
            109 => Self::SERVER_IS_UNDER_MAINTENANCE,
            110 => Self::CHANNEL_IS_BLOCKED,
            112 => Self::NOT_SUBSCRIBED_TO_SPOTS,
            113 => Self::ALREADY_SUBSCRIBED,
            114 => Self::SYMBOL_NOT_FOUND,
            115 => Self::UNKNOWN_SYMBOL,
            117 => Self::NO_QUOTES,
            118 => Self::NOT_ENOUGH_MONEY,
            119 => Self::MAX_EXPOSURE_REACHED,
            120 => Self::POSITION_NOT_FOUND,
            121 => Self::ORDER_NOT_FOUND,
            122 => Self::POSITION_NOT_OPEN,
            123 => Self::POSITION_LOCKED,
            124 => Self::TOO_MANY_POSITIONS,
            125 => Self::TRADING_BAD_VOLUME,
            126 => Self::TRADING_BAD_STOPS,
            127 => Self::TRADING_BAD_PRICES,
            128 => Self::TRADING_BAD_STAKE,
            129 => Self::PROTECTION_IS_TOO_CLOSE_TO_MARKET,
            130 => Self::TRADING_BAD_EXPIRATION_DATE,
            131 => Self::PENDING_EXECUTION,
            132 => Self::TRADING_DISABLED,
            133 => Self::TRADING_NOT_ALLOWED,
            134 => Self::UNABLE_TO_CANCEL_ORDER,
            135 => Self::UNABLE_TO_AMEND_ORDER,
            136 => Self::SHORT_SELLING_NOT_ALLOWED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAErrorCode {
    fn from(s: &'a str) -> Self {
        match s {
            "OA_AUTH_TOKEN_EXPIRED" => Self::OA_AUTH_TOKEN_EXPIRED,
            "ACCOUNT_NOT_AUTHORIZED" => Self::ACCOUNT_NOT_AUTHORIZED,
            "RET_NO_SUCH_LOGIN" => Self::RET_NO_SUCH_LOGIN,
            "ALREADY_LOGGED_IN" => Self::ALREADY_LOGGED_IN,
            "INCORRECT_BOUNDARIES" => Self::INCORRECT_BOUNDARIES,
            "RET_ACCOUNT_DISABLED" => Self::RET_ACCOUNT_DISABLED,
            "CONNECTIONS_LIMIT_EXCEEDED" => Self::CONNECTIONS_LIMIT_EXCEEDED,
            "WORSE_GSL_NOT_ALLOWED" => Self::WORSE_GSL_NOT_ALLOWED,
            "SYMBOL_HAS_HOLIDAY" => Self::SYMBOL_HAS_HOLIDAY,
            "CH_CLIENT_AUTH_FAILURE" => Self::CH_CLIENT_AUTH_FAILURE,
            "CH_CLIENT_NOT_AUTHENTICATED" => Self::CH_CLIENT_NOT_AUTHENTICATED,
            "CH_CLIENT_ALREADY_AUTHENTICATED" => Self::CH_CLIENT_ALREADY_AUTHENTICATED,
            "CH_ACCESS_TOKEN_INVALID" => Self::CH_ACCESS_TOKEN_INVALID,
            "CH_SERVER_NOT_REACHABLE" => Self::CH_SERVER_NOT_REACHABLE,
            "CH_CTID_TRADER_ACCOUNT_NOT_FOUND" => Self::CH_CTID_TRADER_ACCOUNT_NOT_FOUND,
            "CH_OA_CLIENT_NOT_FOUND" => Self::CH_OA_CLIENT_NOT_FOUND,
            "REQUEST_FREQUENCY_EXCEEDED" => Self::REQUEST_FREQUENCY_EXCEEDED,
            "SERVER_IS_UNDER_MAINTENANCE" => Self::SERVER_IS_UNDER_MAINTENANCE,
            "CHANNEL_IS_BLOCKED" => Self::CHANNEL_IS_BLOCKED,
            "NOT_SUBSCRIBED_TO_SPOTS" => Self::NOT_SUBSCRIBED_TO_SPOTS,
            "ALREADY_SUBSCRIBED" => Self::ALREADY_SUBSCRIBED,
            "SYMBOL_NOT_FOUND" => Self::SYMBOL_NOT_FOUND,
            "UNKNOWN_SYMBOL" => Self::UNKNOWN_SYMBOL,
            "NO_QUOTES" => Self::NO_QUOTES,
            "NOT_ENOUGH_MONEY" => Self::NOT_ENOUGH_MONEY,
            "MAX_EXPOSURE_REACHED" => Self::MAX_EXPOSURE_REACHED,
            "POSITION_NOT_FOUND" => Self::POSITION_NOT_FOUND,
            "ORDER_NOT_FOUND" => Self::ORDER_NOT_FOUND,
            "POSITION_NOT_OPEN" => Self::POSITION_NOT_OPEN,
            "POSITION_LOCKED" => Self::POSITION_LOCKED,
            "TOO_MANY_POSITIONS" => Self::TOO_MANY_POSITIONS,
            "TRADING_BAD_VOLUME" => Self::TRADING_BAD_VOLUME,
            "TRADING_BAD_STOPS" => Self::TRADING_BAD_STOPS,
            "TRADING_BAD_PRICES" => Self::TRADING_BAD_PRICES,
            "TRADING_BAD_STAKE" => Self::TRADING_BAD_STAKE,
            "PROTECTION_IS_TOO_CLOSE_TO_MARKET" => Self::PROTECTION_IS_TOO_CLOSE_TO_MARKET,
            "TRADING_BAD_EXPIRATION_DATE" => Self::TRADING_BAD_EXPIRATION_DATE,
            "PENDING_EXECUTION" => Self::PENDING_EXECUTION,
            "TRADING_DISABLED" => Self::TRADING_DISABLED,
            "TRADING_NOT_ALLOWED" => Self::TRADING_NOT_ALLOWED,
            "UNABLE_TO_CANCEL_ORDER" => Self::UNABLE_TO_CANCEL_ORDER,
            "UNABLE_TO_AMEND_ORDER" => Self::UNABLE_TO_AMEND_ORDER,
            "SHORT_SELLING_NOT_ALLOWED" => Self::SHORT_SELLING_NOT_ALLOWED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOALimitedRiskMarginCalculationStrategy {
    ACCORDING_TO_LEVERAGE = 0,
    ACCORDING_TO_GSL = 1,
    ACCORDING_TO_GSL_AND_LEVERAGE = 2,
}

impl Default for ProtoOALimitedRiskMarginCalculationStrategy {
    fn default() -> Self {
        ProtoOALimitedRiskMarginCalculationStrategy::ACCORDING_TO_LEVERAGE
    }
}

impl From<i32> for ProtoOALimitedRiskMarginCalculationStrategy {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::ACCORDING_TO_LEVERAGE,
            1 => Self::ACCORDING_TO_GSL,
            2 => Self::ACCORDING_TO_GSL_AND_LEVERAGE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOALimitedRiskMarginCalculationStrategy {
    fn from(s: &'a str) -> Self {
        match s {
            "ACCORDING_TO_LEVERAGE" => Self::ACCORDING_TO_LEVERAGE,
            "ACCORDING_TO_GSL" => Self::ACCORDING_TO_GSL,
            "ACCORDING_TO_GSL_AND_LEVERAGE" => Self::ACCORDING_TO_GSL_AND_LEVERAGE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAQuoteType {
    BID = 1,
    ASK = 2,
}

impl Default for ProtoOAQuoteType {
    fn default() -> Self {
        ProtoOAQuoteType::BID
    }
}

impl From<i32> for ProtoOAQuoteType {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::BID,
            2 => Self::ASK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAQuoteType {
    fn from(s: &'a str) -> Self {
        match s {
            "BID" => Self::BID,
            "ASK" => Self::ASK,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAClientPermissionScope {
    SCOPE_VIEW = 0,
    SCOPE_TRADE = 1,
}

impl Default for ProtoOAClientPermissionScope {
    fn default() -> Self {
        ProtoOAClientPermissionScope::SCOPE_VIEW
    }
}

impl From<i32> for ProtoOAClientPermissionScope {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::SCOPE_VIEW,
            1 => Self::SCOPE_TRADE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAClientPermissionScope {
    fn from(s: &'a str) -> Self {
        match s {
            "SCOPE_VIEW" => Self::SCOPE_VIEW,
            "SCOPE_TRADE" => Self::SCOPE_TRADE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOATrendbarPeriod {
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
    M10 = 6,
    M15 = 7,
    M30 = 8,
    H1 = 9,
    H4 = 10,
    H12 = 11,
    D1 = 12,
    W1 = 13,
    MN1 = 14,
}

impl Default for ProtoOATrendbarPeriod {
    fn default() -> Self {
        ProtoOATrendbarPeriod::M1
    }
}

impl From<i32> for ProtoOATrendbarPeriod {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::M1,
            2 => Self::M2,
            3 => Self::M3,
            4 => Self::M4,
            5 => Self::M5,
            6 => Self::M10,
            7 => Self::M15,
            8 => Self::M30,
            9 => Self::H1,
            10 => Self::H4,
            11 => Self::H12,
            12 => Self::D1,
            13 => Self::W1,
            14 => Self::MN1,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOATrendbarPeriod {
    fn from(s: &'a str) -> Self {
        match s {
            "M1" => Self::M1,
            "M2" => Self::M2,
            "M3" => Self::M3,
            "M4" => Self::M4,
            "M5" => Self::M5,
            "M10" => Self::M10,
            "M15" => Self::M15,
            "M30" => Self::M30,
            "H1" => Self::H1,
            "H4" => Self::H4,
            "H12" => Self::H12,
            "D1" => Self::D1,
            "W1" => Self::W1,
            "MN1" => Self::MN1,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAPositionStatus {
    POSITION_STATUS_OPEN = 1,
    POSITION_STATUS_CLOSED = 2,
    POSITION_STATUS_CREATED = 3,
    POSITION_STATUS_ERROR = 4,
}

impl Default for ProtoOAPositionStatus {
    fn default() -> Self {
        ProtoOAPositionStatus::POSITION_STATUS_OPEN
    }
}

impl From<i32> for ProtoOAPositionStatus {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::POSITION_STATUS_OPEN,
            2 => Self::POSITION_STATUS_CLOSED,
            3 => Self::POSITION_STATUS_CREATED,
            4 => Self::POSITION_STATUS_ERROR,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAPositionStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "POSITION_STATUS_OPEN" => Self::POSITION_STATUS_OPEN,
            "POSITION_STATUS_CLOSED" => Self::POSITION_STATUS_CLOSED,
            "POSITION_STATUS_CREATED" => Self::POSITION_STATUS_CREATED,
            "POSITION_STATUS_ERROR" => Self::POSITION_STATUS_ERROR,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOATradeSide {
    BUY = 1,
    SELL = 2,
}

impl Default for ProtoOATradeSide {
    fn default() -> Self {
        ProtoOATradeSide::BUY
    }
}

impl From<i32> for ProtoOATradeSide {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::BUY,
            2 => Self::SELL,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOATradeSide {
    fn from(s: &'a str) -> Self {
        match s {
            "BUY" => Self::BUY,
            "SELL" => Self::SELL,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAOrderType {
    MARKET = 1,
    LIMIT = 2,
    STOP = 3,
    STOP_LOSS_TAKE_PROFIT = 4,
    MARKET_RANGE = 5,
    STOP_LIMIT = 6,
}

impl Default for ProtoOAOrderType {
    fn default() -> Self {
        ProtoOAOrderType::MARKET
    }
}

impl From<i32> for ProtoOAOrderType {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::MARKET,
            2 => Self::LIMIT,
            3 => Self::STOP,
            4 => Self::STOP_LOSS_TAKE_PROFIT,
            5 => Self::MARKET_RANGE,
            6 => Self::STOP_LIMIT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAOrderType {
    fn from(s: &'a str) -> Self {
        match s {
            "MARKET" => Self::MARKET,
            "LIMIT" => Self::LIMIT,
            "STOP" => Self::STOP,
            "STOP_LOSS_TAKE_PROFIT" => Self::STOP_LOSS_TAKE_PROFIT,
            "MARKET_RANGE" => Self::MARKET_RANGE,
            "STOP_LIMIT" => Self::STOP_LIMIT,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOATimeInForce {
    GOOD_TILL_DATE = 1,
    GOOD_TILL_CANCEL = 2,
    IMMEDIATE_OR_CANCEL = 3,
    FILL_OR_KILL = 4,
    MARKET_ON_OPEN = 5,
}

impl Default for ProtoOATimeInForce {
    fn default() -> Self {
        ProtoOATimeInForce::GOOD_TILL_DATE
    }
}

impl From<i32> for ProtoOATimeInForce {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::GOOD_TILL_DATE,
            2 => Self::GOOD_TILL_CANCEL,
            3 => Self::IMMEDIATE_OR_CANCEL,
            4 => Self::FILL_OR_KILL,
            5 => Self::MARKET_ON_OPEN,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOATimeInForce {
    fn from(s: &'a str) -> Self {
        match s {
            "GOOD_TILL_DATE" => Self::GOOD_TILL_DATE,
            "GOOD_TILL_CANCEL" => Self::GOOD_TILL_CANCEL,
            "IMMEDIATE_OR_CANCEL" => Self::IMMEDIATE_OR_CANCEL,
            "FILL_OR_KILL" => Self::FILL_OR_KILL,
            "MARKET_ON_OPEN" => Self::MARKET_ON_OPEN,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAOrderStatus {
    ORDER_STATUS_ACCEPTED = 1,
    ORDER_STATUS_FILLED = 2,
    ORDER_STATUS_REJECTED = 3,
    ORDER_STATUS_EXPIRED = 4,
    ORDER_STATUS_CANCELLED = 5,
}

impl Default for ProtoOAOrderStatus {
    fn default() -> Self {
        ProtoOAOrderStatus::ORDER_STATUS_ACCEPTED
    }
}

impl From<i32> for ProtoOAOrderStatus {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::ORDER_STATUS_ACCEPTED,
            2 => Self::ORDER_STATUS_FILLED,
            3 => Self::ORDER_STATUS_REJECTED,
            4 => Self::ORDER_STATUS_EXPIRED,
            5 => Self::ORDER_STATUS_CANCELLED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAOrderStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "ORDER_STATUS_ACCEPTED" => Self::ORDER_STATUS_ACCEPTED,
            "ORDER_STATUS_FILLED" => Self::ORDER_STATUS_FILLED,
            "ORDER_STATUS_REJECTED" => Self::ORDER_STATUS_REJECTED,
            "ORDER_STATUS_EXPIRED" => Self::ORDER_STATUS_EXPIRED,
            "ORDER_STATUS_CANCELLED" => Self::ORDER_STATUS_CANCELLED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAOrderTriggerMethod {
    TRADE = 1,
    OPPOSITE = 2,
    DOUBLE_TRADE = 3,
    DOUBLE_OPPOSITE = 4,
}

impl Default for ProtoOAOrderTriggerMethod {
    fn default() -> Self {
        ProtoOAOrderTriggerMethod::TRADE
    }
}

impl From<i32> for ProtoOAOrderTriggerMethod {
    fn from(i: i32) -> Self {
        match i {
            1 => Self::TRADE,
            2 => Self::OPPOSITE,
            3 => Self::DOUBLE_TRADE,
            4 => Self::DOUBLE_OPPOSITE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAOrderTriggerMethod {
    fn from(s: &'a str) -> Self {
        match s {
            "TRADE" => Self::TRADE,
            "OPPOSITE" => Self::OPPOSITE,
            "DOUBLE_TRADE" => Self::DOUBLE_TRADE,
            "DOUBLE_OPPOSITE" => Self::DOUBLE_OPPOSITE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAExecutionType {
    ORDER_ACCEPTED = 2,
    ORDER_FILLED = 3,
    ORDER_REPLACED = 4,
    ORDER_CANCELLED = 5,
    ORDER_EXPIRED = 6,
    ORDER_REJECTED = 7,
    ORDER_CANCEL_REJECTED = 8,
    SWAP = 9,
    DEPOSIT_WITHDRAW = 10,
    ORDER_PARTIAL_FILL = 11,
    BONUS_DEPOSIT_WITHDRAW = 12,
}

impl Default for ProtoOAExecutionType {
    fn default() -> Self {
        ProtoOAExecutionType::ORDER_ACCEPTED
    }
}

impl From<i32> for ProtoOAExecutionType {
    fn from(i: i32) -> Self {
        match i {
            2 => Self::ORDER_ACCEPTED,
            3 => Self::ORDER_FILLED,
            4 => Self::ORDER_REPLACED,
            5 => Self::ORDER_CANCELLED,
            6 => Self::ORDER_EXPIRED,
            7 => Self::ORDER_REJECTED,
            8 => Self::ORDER_CANCEL_REJECTED,
            9 => Self::SWAP,
            10 => Self::DEPOSIT_WITHDRAW,
            11 => Self::ORDER_PARTIAL_FILL,
            12 => Self::BONUS_DEPOSIT_WITHDRAW,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAExecutionType {
    fn from(s: &'a str) -> Self {
        match s {
            "ORDER_ACCEPTED" => Self::ORDER_ACCEPTED,
            "ORDER_FILLED" => Self::ORDER_FILLED,
            "ORDER_REPLACED" => Self::ORDER_REPLACED,
            "ORDER_CANCELLED" => Self::ORDER_CANCELLED,
            "ORDER_EXPIRED" => Self::ORDER_EXPIRED,
            "ORDER_REJECTED" => Self::ORDER_REJECTED,
            "ORDER_CANCEL_REJECTED" => Self::ORDER_CANCEL_REJECTED,
            "SWAP" => Self::SWAP,
            "DEPOSIT_WITHDRAW" => Self::DEPOSIT_WITHDRAW,
            "ORDER_PARTIAL_FILL" => Self::ORDER_PARTIAL_FILL,
            "BONUS_DEPOSIT_WITHDRAW" => Self::BONUS_DEPOSIT_WITHDRAW,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAChangeBalanceType {
    BALANCE_DEPOSIT = 0,
    BALANCE_WITHDRAW = 1,
    BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER = 3,
    BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER = 4,
    BALANCE_DEPOSIT_IB_COMMISSIONS = 5,
    BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE = 6,
    BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB = 7,
    BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER = 8,
    BALANCE_DEPOSIT_REBATE = 9,
    BALANCE_WITHDRAW_REBATE = 10,
    BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER = 11,
    BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER = 12,
    BALANCE_WITHDRAW_BONUS_COMPENSATION = 13,
    BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER = 14,
    BALANCE_DEPOSIT_DIVIDENDS = 15,
    BALANCE_WITHDRAW_DIVIDENDS = 16,
    BALANCE_WITHDRAW_GSL_CHARGE = 17,
    BALANCE_WITHDRAW_ROLLOVER = 18,
    BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS = 19,
    BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS = 20,
    BALANCE_DEPOSIT_SWAP = 21,
    BALANCE_WITHDRAW_SWAP = 22,
    BALANCE_DEPOSIT_MANAGEMENT_FEE = 27,
    BALANCE_DEPOSIT_PERFORMANCE_FEE = 29,
    BALANCE_WITHDRAW_FOR_SUBACCOUNT = 30,
    BALANCE_DEPOSIT_TO_SUBACCOUNT = 31,
    BALANCE_WITHDRAW_FROM_SUBACCOUNT = 32,
    BALANCE_DEPOSIT_FROM_SUBACCOUNT = 33,
    BALANCE_WITHDRAW_COPY_FEE = 34,
    BALANCE_WITHDRAW_INACTIVITY_FEE = 35,
    BALANCE_DEPOSIT_TRANSFER = 36,
    BALANCE_WITHDRAW_TRANSFER = 37,
    BALANCE_DEPOSIT_CONVERTED_BONUS = 38,
    BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION = 39,
}

impl Default for ProtoOAChangeBalanceType {
    fn default() -> Self {
        ProtoOAChangeBalanceType::BALANCE_DEPOSIT
    }
}

impl From<i32> for ProtoOAChangeBalanceType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::BALANCE_DEPOSIT,
            1 => Self::BALANCE_WITHDRAW,
            3 => Self::BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER,
            4 => Self::BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER,
            5 => Self::BALANCE_DEPOSIT_IB_COMMISSIONS,
            6 => Self::BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE,
            7 => Self::BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB,
            8 => Self::BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER,
            9 => Self::BALANCE_DEPOSIT_REBATE,
            10 => Self::BALANCE_WITHDRAW_REBATE,
            11 => Self::BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER,
            12 => Self::BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER,
            13 => Self::BALANCE_WITHDRAW_BONUS_COMPENSATION,
            14 => Self::BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER,
            15 => Self::BALANCE_DEPOSIT_DIVIDENDS,
            16 => Self::BALANCE_WITHDRAW_DIVIDENDS,
            17 => Self::BALANCE_WITHDRAW_GSL_CHARGE,
            18 => Self::BALANCE_WITHDRAW_ROLLOVER,
            19 => Self::BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS,
            20 => Self::BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS,
            21 => Self::BALANCE_DEPOSIT_SWAP,
            22 => Self::BALANCE_WITHDRAW_SWAP,
            27 => Self::BALANCE_DEPOSIT_MANAGEMENT_FEE,
            29 => Self::BALANCE_DEPOSIT_PERFORMANCE_FEE,
            30 => Self::BALANCE_WITHDRAW_FOR_SUBACCOUNT,
            31 => Self::BALANCE_DEPOSIT_TO_SUBACCOUNT,
            32 => Self::BALANCE_WITHDRAW_FROM_SUBACCOUNT,
            33 => Self::BALANCE_DEPOSIT_FROM_SUBACCOUNT,
            34 => Self::BALANCE_WITHDRAW_COPY_FEE,
            35 => Self::BALANCE_WITHDRAW_INACTIVITY_FEE,
            36 => Self::BALANCE_DEPOSIT_TRANSFER,
            37 => Self::BALANCE_WITHDRAW_TRANSFER,
            38 => Self::BALANCE_DEPOSIT_CONVERTED_BONUS,
            39 => Self::BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAChangeBalanceType {
    fn from(s: &'a str) -> Self {
        match s {
            "BALANCE_DEPOSIT" => Self::BALANCE_DEPOSIT,
            "BALANCE_WITHDRAW" => Self::BALANCE_WITHDRAW,
            "BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER" => Self::BALANCE_DEPOSIT_STRATEGY_COMMISSION_INNER,
            "BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER" => Self::BALANCE_WITHDRAW_STRATEGY_COMMISSION_INNER,
            "BALANCE_DEPOSIT_IB_COMMISSIONS" => Self::BALANCE_DEPOSIT_IB_COMMISSIONS,
            "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE" => Self::BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE,
            "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB" => Self::BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_SUB_IB,
            "BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER" => Self::BALANCE_DEPOSIT_IB_SHARED_PERCENTAGE_FROM_BROKER,
            "BALANCE_DEPOSIT_REBATE" => Self::BALANCE_DEPOSIT_REBATE,
            "BALANCE_WITHDRAW_REBATE" => Self::BALANCE_WITHDRAW_REBATE,
            "BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER" => Self::BALANCE_DEPOSIT_STRATEGY_COMMISSION_OUTER,
            "BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER" => Self::BALANCE_WITHDRAW_STRATEGY_COMMISSION_OUTER,
            "BALANCE_WITHDRAW_BONUS_COMPENSATION" => Self::BALANCE_WITHDRAW_BONUS_COMPENSATION,
            "BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER" => Self::BALANCE_WITHDRAW_IB_SHARED_PERCENTAGE_TO_BROKER,
            "BALANCE_DEPOSIT_DIVIDENDS" => Self::BALANCE_DEPOSIT_DIVIDENDS,
            "BALANCE_WITHDRAW_DIVIDENDS" => Self::BALANCE_WITHDRAW_DIVIDENDS,
            "BALANCE_WITHDRAW_GSL_CHARGE" => Self::BALANCE_WITHDRAW_GSL_CHARGE,
            "BALANCE_WITHDRAW_ROLLOVER" => Self::BALANCE_WITHDRAW_ROLLOVER,
            "BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS" => Self::BALANCE_DEPOSIT_NONWITHDRAWABLE_BONUS,
            "BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS" => Self::BALANCE_WITHDRAW_NONWITHDRAWABLE_BONUS,
            "BALANCE_DEPOSIT_SWAP" => Self::BALANCE_DEPOSIT_SWAP,
            "BALANCE_WITHDRAW_SWAP" => Self::BALANCE_WITHDRAW_SWAP,
            "BALANCE_DEPOSIT_MANAGEMENT_FEE" => Self::BALANCE_DEPOSIT_MANAGEMENT_FEE,
            "BALANCE_DEPOSIT_PERFORMANCE_FEE" => Self::BALANCE_DEPOSIT_PERFORMANCE_FEE,
            "BALANCE_WITHDRAW_FOR_SUBACCOUNT" => Self::BALANCE_WITHDRAW_FOR_SUBACCOUNT,
            "BALANCE_DEPOSIT_TO_SUBACCOUNT" => Self::BALANCE_DEPOSIT_TO_SUBACCOUNT,
            "BALANCE_WITHDRAW_FROM_SUBACCOUNT" => Self::BALANCE_WITHDRAW_FROM_SUBACCOUNT,
            "BALANCE_DEPOSIT_FROM_SUBACCOUNT" => Self::BALANCE_DEPOSIT_FROM_SUBACCOUNT,
            "BALANCE_WITHDRAW_COPY_FEE" => Self::BALANCE_WITHDRAW_COPY_FEE,
            "BALANCE_WITHDRAW_INACTIVITY_FEE" => Self::BALANCE_WITHDRAW_INACTIVITY_FEE,
            "BALANCE_DEPOSIT_TRANSFER" => Self::BALANCE_DEPOSIT_TRANSFER,
            "BALANCE_WITHDRAW_TRANSFER" => Self::BALANCE_WITHDRAW_TRANSFER,
            "BALANCE_DEPOSIT_CONVERTED_BONUS" => Self::BALANCE_DEPOSIT_CONVERTED_BONUS,
            "BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION" => Self::BALANCE_DEPOSIT_NEGATIVE_BALANCE_PROTECTION,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOADealStatus {
    FILLED = 2,
    PARTIALLY_FILLED = 3,
    REJECTED = 4,
    INTERNALLY_REJECTED = 5,
    ERROR = 6,
    MISSED = 7,
}

impl Default for ProtoOADealStatus {
    fn default() -> Self {
        ProtoOADealStatus::FILLED
    }
}

impl From<i32> for ProtoOADealStatus {
    fn from(i: i32) -> Self {
        match i {
            2 => Self::FILLED,
            3 => Self::PARTIALLY_FILLED,
            4 => Self::REJECTED,
            5 => Self::INTERNALLY_REJECTED,
            6 => Self::ERROR,
            7 => Self::MISSED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOADealStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "FILLED" => Self::FILLED,
            "PARTIALLY_FILLED" => Self::PARTIALLY_FILLED,
            "REJECTED" => Self::REJECTED,
            "INTERNALLY_REJECTED" => Self::INTERNALLY_REJECTED,
            "ERROR" => Self::ERROR,
            "MISSED" => Self::MISSED,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAChangeBonusType {
    BONUS_DEPOSIT = 0,
    BONUS_WITHDRAW = 1,
}

impl Default for ProtoOAChangeBonusType {
    fn default() -> Self {
        ProtoOAChangeBonusType::BONUS_DEPOSIT
    }
}

impl From<i32> for ProtoOAChangeBonusType {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::BONUS_DEPOSIT,
            1 => Self::BONUS_WITHDRAW,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAChangeBonusType {
    fn from(s: &'a str) -> Self {
        match s {
            "BONUS_DEPOSIT" => Self::BONUS_DEPOSIT,
            "BONUS_WITHDRAW" => Self::BONUS_WITHDRAW,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ProtoOAStopOutStrategy {
    MOST_MARGIN_USED_FIRST = 0,
    MOST_LOSING_FIRST = 1,
}

impl Default for ProtoOAStopOutStrategy {
    fn default() -> Self {
        ProtoOAStopOutStrategy::MOST_MARGIN_USED_FIRST
    }
}

impl From<i32> for ProtoOAStopOutStrategy {
    fn from(i: i32) -> Self {
        match i {
            0 => Self::MOST_MARGIN_USED_FIRST,
            1 => Self::MOST_LOSING_FIRST,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ProtoOAStopOutStrategy {
    fn from(s: &'a str) -> Self {
        match s {
            "MOST_MARGIN_USED_FIRST" => Self::MOST_MARGIN_USED_FIRST,
            "MOST_LOSING_FIRST" => Self::MOST_LOSING_FIRST,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoMessage<'a> {
    pub payloadType: u32,
    pub payload: Option<Cow<'a, [u8]>>,
    pub clientMsgId: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = r.read_uint32(bytes)?,
                Ok(18) => msg.payload = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.clientMsgId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoMessage<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.payloadType) as u64)
        + self.payload.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.clientMsgId.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.payloadType))?;
        self.payload.as_ref().map_or(Ok(()), |m| w.write_with_tag(18, |w| w.write_bytes(&m)))?;
        self.clientMsgId.as_ref().map_or(Ok(()), |m| w.write_with_tag(26, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoErrorRes<'a> {
    pub payloadType: Option<spotware::ProtoPayloadType>,
    pub errorCode: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub maintenanceEndTimestamp: Option<u64>,
}

impl<'a> MessageRead<'a> for ProtoErrorRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.errorCode = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.maintenanceEndTimestamp = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoErrorRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.errorCode).len())
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.maintenanceEndTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.errorCode))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(26, |w| w.write_string(&m)))?;
        self.maintenanceEndTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_uint64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoHeartbeatEvent {
    pub payloadType: Option<spotware::ProtoPayloadType>,
}

impl<'a> MessageRead<'a> for ProtoHeartbeatEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoHeartbeatEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAsset<'a> {
    pub assetId: i64,
    pub name: Cow<'a, str>,
    pub displayName: Option<Cow<'a, str>>,
    pub digits: Option<i32>,
}

impl<'a> MessageRead<'a> for ProtoOAAsset<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.assetId = r.read_int64(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.displayName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.digits = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAsset<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.assetId) as u64)
        + 1 + sizeof_len((&self.name).len())
        + self.displayName.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.digits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.assetId))?;
        w.write_with_tag(18, |w| w.write_string(&self.name))?;
        self.displayName.as_ref().map_or(Ok(()), |m| w.write_with_tag(26, |w| w.write_string(&m)))?;
        self.digits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAInterval {
    pub startSecond: u32,
    pub endSecond: u32,
}

impl<'a> MessageRead<'a> for ProtoOAInterval {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(24) => msg.startSecond = r.read_uint32(bytes)?,
                Ok(32) => msg.endSecond = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAInterval {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.startSecond) as u64)
        + 1 + sizeof_varint(*(&self.endSecond) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(24, |w| w.write_uint32(*&self.startSecond))?;
        w.write_with_tag(32, |w| w.write_uint32(*&self.endSecond))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbol<'a> {
    pub symbolId: i64,
    pub digits: i32,
    pub pipPosition: i32,
    pub enableShortSelling: Option<bool>,
    pub guaranteedStopLoss: Option<bool>,
    pub swapRollover3Days: Option<spotware::ProtoOADayOfWeek>,
    pub swapLong: Option<f64>,
    pub swapShort: Option<f64>,
    pub maxVolume: Option<i64>,
    pub minVolume: Option<i64>,
    pub stepVolume: Option<i64>,
    pub maxExposure: Option<u64>,
    pub schedule: Vec<spotware::ProtoOAInterval>,
    pub commissionType: Option<spotware::ProtoOACommissionType>,
    pub slDistance: Option<u32>,
    pub tpDistance: Option<u32>,
    pub gslDistance: Option<u32>,
    pub gslCharge: Option<i64>,
    pub distanceSetIn: Option<spotware::ProtoOASymbolDistanceType>,
    pub minCommissionType: Option<spotware::ProtoOAMinCommissionType>,
    pub minCommissionAsset: Option<Cow<'a, str>>,
    pub rolloverCommission: Option<i64>,
    pub skipRolloverDays: Option<i32>,
    pub scheduleTimeZone: Option<Cow<'a, str>>,
    pub tradingMode: Option<spotware::ProtoOATradingMode>,
    pub rolloverCommission3Days: Option<spotware::ProtoOADayOfWeek>,
    pub swapCalculationType: Option<spotware::ProtoOASwapCalculationType>,
    pub lotSize: Option<i64>,
    pub preciseTradingCommissionRate: Option<i64>,
    pub preciseMinCommission: Option<i64>,
    pub holiday: Vec<spotware::ProtoOAHoliday<'a>>,
    pub pnlConversionFeeRate: Option<i32>,
    pub leverageId: Option<i64>,
    pub swapPeriod: Option<i32>,
    pub swapTime: Option<i32>,
    pub skipSWAPPeriods: Option<i32>,
    pub chargeSwapAtWeekends: Option<bool>,
    pub measurementUnits: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOASymbol<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.symbolId = r.read_int64(bytes)?,
                Ok(16) => msg.digits = r.read_int32(bytes)?,
                Ok(24) => msg.pipPosition = r.read_int32(bytes)?,
                Ok(32) => msg.enableShortSelling = Some(r.read_bool(bytes)?),
                Ok(40) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(48) => msg.swapRollover3Days = Some(r.read_enum(bytes)?),
                Ok(57) => msg.swapLong = Some(r.read_double(bytes)?),
                Ok(65) => msg.swapShort = Some(r.read_double(bytes)?),
                Ok(72) => msg.maxVolume = Some(r.read_int64(bytes)?),
                Ok(80) => msg.minVolume = Some(r.read_int64(bytes)?),
                Ok(88) => msg.stepVolume = Some(r.read_int64(bytes)?),
                Ok(96) => msg.maxExposure = Some(r.read_uint64(bytes)?),
                Ok(106) => msg.schedule.push(r.read_message::<spotware::ProtoOAInterval>(bytes)?),
                Ok(120) => msg.commissionType = Some(r.read_enum(bytes)?),
                Ok(128) => msg.slDistance = Some(r.read_uint32(bytes)?),
                Ok(136) => msg.tpDistance = Some(r.read_uint32(bytes)?),
                Ok(144) => msg.gslDistance = Some(r.read_uint32(bytes)?),
                Ok(152) => msg.gslCharge = Some(r.read_int64(bytes)?),
                Ok(160) => msg.distanceSetIn = Some(r.read_enum(bytes)?),
                Ok(176) => msg.minCommissionType = Some(r.read_enum(bytes)?),
                Ok(186) => msg.minCommissionAsset = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(192) => msg.rolloverCommission = Some(r.read_int64(bytes)?),
                Ok(200) => msg.skipRolloverDays = Some(r.read_int32(bytes)?),
                Ok(210) => msg.scheduleTimeZone = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(216) => msg.tradingMode = Some(r.read_enum(bytes)?),
                Ok(224) => msg.rolloverCommission3Days = Some(r.read_enum(bytes)?),
                Ok(232) => msg.swapCalculationType = Some(r.read_enum(bytes)?),
                Ok(240) => msg.lotSize = Some(r.read_int64(bytes)?),
                Ok(248) => msg.preciseTradingCommissionRate = Some(r.read_int64(bytes)?),
                Ok(256) => msg.preciseMinCommission = Some(r.read_int64(bytes)?),
                Ok(266) => msg.holiday.push(r.read_message::<spotware::ProtoOAHoliday>(bytes)?),
                Ok(272) => msg.pnlConversionFeeRate = Some(r.read_int32(bytes)?),
                Ok(280) => msg.leverageId = Some(r.read_int64(bytes)?),
                Ok(288) => msg.swapPeriod = Some(r.read_int32(bytes)?),
                Ok(296) => msg.swapTime = Some(r.read_int32(bytes)?),
                Ok(304) => msg.skipSWAPPeriods = Some(r.read_int32(bytes)?),
                Ok(312) => msg.chargeSwapAtWeekends = Some(r.read_bool(bytes)?),
                Ok(322) => msg.measurementUnits = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbol<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_varint(*(&self.digits) as u64)
        + 1 + sizeof_varint(*(&self.pipPosition) as u64)
        + self.enableShortSelling.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.swapRollover3Days.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.swapLong.as_ref().map_or(0, |&m| 1 + 8)
        + self.swapShort.as_ref().map_or(0, |&m| 1 + 8)
        + self.maxVolume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.minVolume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stepVolume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.maxExposure.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.schedule.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.commissionType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.slDistance.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.tpDistance.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.gslDistance.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.gslCharge.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.distanceSetIn.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.minCommissionType.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.minCommissionAsset.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.rolloverCommission.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.skipRolloverDays.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.scheduleTimeZone.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.tradingMode.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.rolloverCommission3Days.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.swapCalculationType.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.lotSize.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.preciseTradingCommissionRate.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.preciseMinCommission.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.holiday.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.pnlConversionFeeRate.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.leverageId.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.swapPeriod.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.swapTime.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.skipSWAPPeriods.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.chargeSwapAtWeekends.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.measurementUnits.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.digits))?;
        w.write_with_tag(24, |w| w.write_int32(*&self.pipPosition))?;
        self.enableShortSelling.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_bool(*&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_bool(*&m)))?;
        self.swapRollover3Days.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_enum(*&m as i32)))?;
        self.swapLong.as_ref().map_or(Ok(()), |&m| w.write_with_tag(57, |w| w.write_double(*&m)))?;
        self.swapShort.as_ref().map_or(Ok(()), |&m| w.write_with_tag(65, |w| w.write_double(*&m)))?;
        self.maxVolume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_int64(*&m)))?;
        self.minVolume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_int64(*&m)))?;
        self.stepVolume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(88, |w| w.write_int64(*&m)))?;
        self.maxExposure.as_ref().map_or(Ok(()), |&m| w.write_with_tag(96, |w| w.write_uint64(*&m)))?;
        for s in &self.schedule { w.write_with_tag(106, |w| w.write_message(s))?; }
        self.commissionType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(120, |w| w.write_enum(*&m as i32)))?;
        self.slDistance.as_ref().map_or(Ok(()), |&m| w.write_with_tag(128, |w| w.write_uint32(*&m)))?;
        self.tpDistance.as_ref().map_or(Ok(()), |&m| w.write_with_tag(136, |w| w.write_uint32(*&m)))?;
        self.gslDistance.as_ref().map_or(Ok(()), |&m| w.write_with_tag(144, |w| w.write_uint32(*&m)))?;
        self.gslCharge.as_ref().map_or(Ok(()), |&m| w.write_with_tag(152, |w| w.write_int64(*&m)))?;
        self.distanceSetIn.as_ref().map_or(Ok(()), |&m| w.write_with_tag(160, |w| w.write_enum(*&m as i32)))?;
        self.minCommissionType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(176, |w| w.write_enum(*&m as i32)))?;
        self.minCommissionAsset.as_ref().map_or(Ok(()), |m| w.write_with_tag(186, |w| w.write_string(&m)))?;
        self.rolloverCommission.as_ref().map_or(Ok(()), |&m| w.write_with_tag(192, |w| w.write_int64(*&m)))?;
        self.skipRolloverDays.as_ref().map_or(Ok(()), |&m| w.write_with_tag(200, |w| w.write_int32(*&m)))?;
        self.scheduleTimeZone.as_ref().map_or(Ok(()), |m| w.write_with_tag(210, |w| w.write_string(&m)))?;
        self.tradingMode.as_ref().map_or(Ok(()), |&m| w.write_with_tag(216, |w| w.write_enum(*&m as i32)))?;
        self.rolloverCommission3Days.as_ref().map_or(Ok(()), |&m| w.write_with_tag(224, |w| w.write_enum(*&m as i32)))?;
        self.swapCalculationType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(232, |w| w.write_enum(*&m as i32)))?;
        self.lotSize.as_ref().map_or(Ok(()), |&m| w.write_with_tag(240, |w| w.write_int64(*&m)))?;
        self.preciseTradingCommissionRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(248, |w| w.write_int64(*&m)))?;
        self.preciseMinCommission.as_ref().map_or(Ok(()), |&m| w.write_with_tag(256, |w| w.write_int64(*&m)))?;
        for s in &self.holiday { w.write_with_tag(266, |w| w.write_message(s))?; }
        self.pnlConversionFeeRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(272, |w| w.write_int32(*&m)))?;
        self.leverageId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(280, |w| w.write_int64(*&m)))?;
        self.swapPeriod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(288, |w| w.write_int32(*&m)))?;
        self.swapTime.as_ref().map_or(Ok(()), |&m| w.write_with_tag(296, |w| w.write_int32(*&m)))?;
        self.skipSWAPPeriods.as_ref().map_or(Ok(()), |&m| w.write_with_tag(304, |w| w.write_int32(*&m)))?;
        self.chargeSwapAtWeekends.as_ref().map_or(Ok(()), |&m| w.write_with_tag(312, |w| w.write_bool(*&m)))?;
        self.measurementUnits.as_ref().map_or(Ok(()), |m| w.write_with_tag(322, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOALightSymbol<'a> {
    pub symbolId: i64,
    pub symbolName: Option<Cow<'a, str>>,
    pub enabled: Option<bool>,
    pub baseAssetId: Option<i64>,
    pub quoteAssetId: Option<i64>,
    pub symbolCategoryId: Option<i64>,
    pub description: Option<Cow<'a, str>>,
    pub sortingNumber: Option<f64>,
}

impl<'a> MessageRead<'a> for ProtoOALightSymbol<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.symbolId = r.read_int64(bytes)?,
                Ok(18) => msg.symbolName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.enabled = Some(r.read_bool(bytes)?),
                Ok(32) => msg.baseAssetId = Some(r.read_int64(bytes)?),
                Ok(40) => msg.quoteAssetId = Some(r.read_int64(bytes)?),
                Ok(48) => msg.symbolCategoryId = Some(r.read_int64(bytes)?),
                Ok(58) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(65) => msg.sortingNumber = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOALightSymbol<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.symbolId) as u64)
        + self.symbolName.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.enabled.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.baseAssetId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.quoteAssetId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.symbolCategoryId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.sortingNumber.as_ref().map_or(0, |&m| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.symbolId))?;
        self.symbolName.as_ref().map_or(Ok(()), |m| w.write_with_tag(18, |w| w.write_string(&m)))?;
        self.enabled.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_bool(*&m)))?;
        self.baseAssetId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.quoteAssetId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.symbolCategoryId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(58, |w| w.write_string(&m)))?;
        self.sortingNumber.as_ref().map_or(Ok(()), |&m| w.write_with_tag(65, |w| w.write_double(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAArchivedSymbol<'a> {
    pub symbolId: i64,
    pub name: Cow<'a, str>,
    pub utcLastUpdateTimestamp: i64,
    pub description: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOAArchivedSymbol<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.symbolId = r.read_int64(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.utcLastUpdateTimestamp = r.read_int64(bytes)?,
                Ok(34) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAArchivedSymbol<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_varint(*(&self.utcLastUpdateTimestamp) as u64)
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(18, |w| w.write_string(&self.name))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.utcLastUpdateTimestamp))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(34, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolCategory<'a> {
    pub id: i64,
    pub assetClassId: i64,
    pub name: Cow<'a, str>,
    pub sortingNumber: Option<f64>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolCategory<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_int64(bytes)?,
                Ok(16) => msg.assetClassId = r.read_int64(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(33) => msg.sortingNumber = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbolCategory<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_varint(*(&self.assetClassId) as u64)
        + 1 + sizeof_len((&self.name).len())
        + self.sortingNumber.as_ref().map_or(0, |&m| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.id))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.assetClassId))?;
        w.write_with_tag(26, |w| w.write_string(&self.name))?;
        self.sortingNumber.as_ref().map_or(Ok(()), |&m| w.write_with_tag(33, |w| w.write_double(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATrader<'a> {
    pub ctidTraderAccountId: i64,
    pub balance: i64,
    pub balanceVersion: Option<i64>,
    pub managerBonus: Option<i64>,
    pub ibBonus: Option<i64>,
    pub nonWithdrawableBonus: Option<i64>,
    pub accessRights: Option<spotware::ProtoOAAccessRights>,
    pub depositAssetId: i64,
    pub swapFree: Option<bool>,
    pub leverageInCents: Option<u32>,
    pub totalMarginCalculationType: Option<spotware::ProtoOATotalMarginCalculationType>,
    pub maxLeverage: Option<u32>,
    pub traderLogin: Option<i64>,
    pub accountType: Option<spotware::ProtoOAAccountType>,
    pub brokerName: Option<Cow<'a, str>>,
    pub registrationTimestamp: Option<i64>,
    pub isLimitedRisk: Option<bool>,
    pub limitedRiskMarginCalculationStrategy: Option<spotware::ProtoOALimitedRiskMarginCalculationStrategy>,
    pub moneyDigits: Option<u32>,
    pub fairStopOut: Option<bool>,
    pub stopOutStrategy: Option<spotware::ProtoOAStopOutStrategy>,
}

impl<'a> MessageRead<'a> for ProtoOATrader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(16) => msg.balance = r.read_int64(bytes)?,
                Ok(24) => msg.balanceVersion = Some(r.read_int64(bytes)?),
                Ok(32) => msg.managerBonus = Some(r.read_int64(bytes)?),
                Ok(40) => msg.ibBonus = Some(r.read_int64(bytes)?),
                Ok(48) => msg.nonWithdrawableBonus = Some(r.read_int64(bytes)?),
                Ok(56) => msg.accessRights = Some(r.read_enum(bytes)?),
                Ok(64) => msg.depositAssetId = r.read_int64(bytes)?,
                Ok(72) => msg.swapFree = Some(r.read_bool(bytes)?),
                Ok(80) => msg.leverageInCents = Some(r.read_uint32(bytes)?),
                Ok(88) => msg.totalMarginCalculationType = Some(r.read_enum(bytes)?),
                Ok(96) => msg.maxLeverage = Some(r.read_uint32(bytes)?),
                Ok(112) => msg.traderLogin = Some(r.read_int64(bytes)?),
                Ok(120) => msg.accountType = Some(r.read_enum(bytes)?),
                Ok(130) => msg.brokerName = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(136) => msg.registrationTimestamp = Some(r.read_int64(bytes)?),
                Ok(144) => msg.isLimitedRisk = Some(r.read_bool(bytes)?),
                Ok(152) => msg.limitedRiskMarginCalculationStrategy = Some(r.read_enum(bytes)?),
                Ok(160) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(168) => msg.fairStopOut = Some(r.read_bool(bytes)?),
                Ok(176) => msg.stopOutStrategy = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOATrader<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.balance) as u64)
        + self.balanceVersion.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.managerBonus.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ibBonus.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.nonWithdrawableBonus.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.accessRights.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.depositAssetId) as u64)
        + self.swapFree.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.leverageInCents.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.totalMarginCalculationType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.maxLeverage.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.traderLogin.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.accountType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.brokerName.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.registrationTimestamp.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.isLimitedRisk.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.limitedRiskMarginCalculationStrategy.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.moneyDigits.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.fairStopOut.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.stopOutStrategy.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.balance))?;
        self.balanceVersion.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.managerBonus.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.ibBonus.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.nonWithdrawableBonus.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        self.accessRights.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(64, |w| w.write_int64(*&self.depositAssetId))?;
        self.swapFree.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_bool(*&m)))?;
        self.leverageInCents.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_uint32(*&m)))?;
        self.totalMarginCalculationType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(88, |w| w.write_enum(*&m as i32)))?;
        self.maxLeverage.as_ref().map_or(Ok(()), |&m| w.write_with_tag(96, |w| w.write_uint32(*&m)))?;
        self.traderLogin.as_ref().map_or(Ok(()), |&m| w.write_with_tag(112, |w| w.write_int64(*&m)))?;
        self.accountType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(120, |w| w.write_enum(*&m as i32)))?;
        self.brokerName.as_ref().map_or(Ok(()), |m| w.write_with_tag(130, |w| w.write_string(&m)))?;
        self.registrationTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(136, |w| w.write_int64(*&m)))?;
        self.isLimitedRisk.as_ref().map_or(Ok(()), |&m| w.write_with_tag(144, |w| w.write_bool(*&m)))?;
        self.limitedRiskMarginCalculationStrategy.as_ref().map_or(Ok(()), |&m| w.write_with_tag(152, |w| w.write_enum(*&m as i32)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(160, |w| w.write_uint32(*&m)))?;
        self.fairStopOut.as_ref().map_or(Ok(()), |&m| w.write_with_tag(168, |w| w.write_bool(*&m)))?;
        self.stopOutStrategy.as_ref().map_or(Ok(()), |&m| w.write_with_tag(176, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATradeData<'a> {
    pub symbolId: i64,
    pub volume: i64,
    pub tradeSide: spotware::ProtoOATradeSide,
    pub openTimestamp: Option<i64>,
    pub label: Option<Cow<'a, str>>,
    pub guaranteedStopLoss: Option<bool>,
    pub comment: Option<Cow<'a, str>>,
    pub measurementUnits: Option<Cow<'a, str>>,
    pub closeTimestamp: Option<u64>,
}

impl<'a> MessageRead<'a> for ProtoOATradeData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.symbolId = r.read_int64(bytes)?,
                Ok(16) => msg.volume = r.read_int64(bytes)?,
                Ok(24) => msg.tradeSide = r.read_enum(bytes)?,
                Ok(32) => msg.openTimestamp = Some(r.read_int64(bytes)?),
                Ok(42) => msg.label = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(58) => msg.comment = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(66) => msg.measurementUnits = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.closeTimestamp = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOATradeData<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_varint(*(&self.volume) as u64)
        + 1 + sizeof_varint(*(&self.tradeSide) as u64)
        + self.openTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.label.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.comment.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.measurementUnits.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.closeTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.volume))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.tradeSide as i32))?;
        self.openTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.label.as_ref().map_or(Ok(()), |m| w.write_with_tag(42, |w| w.write_string(&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_bool(*&m)))?;
        self.comment.as_ref().map_or(Ok(()), |m| w.write_with_tag(58, |w| w.write_string(&m)))?;
        self.measurementUnits.as_ref().map_or(Ok(()), |m| w.write_with_tag(66, |w| w.write_string(&m)))?;
        self.closeTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_uint64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAPosition<'a> {
    pub positionId: i64,
    pub tradeData: spotware::ProtoOATradeData<'a>,
    pub positionStatus: spotware::ProtoOAPositionStatus,
    pub swap: i64,
    pub price: Option<f64>,
    pub stopLoss: Option<f64>,
    pub takeProfit: Option<f64>,
    pub utcLastUpdateTimestamp: Option<i64>,
    pub commission: Option<i64>,
    pub marginRate: Option<f64>,
    pub mirroringCommission: Option<i64>,
    pub guaranteedStopLoss: Option<bool>,
    pub usedMargin: Option<u64>,
    pub stopLossTriggerMethod: Option<spotware::ProtoOAOrderTriggerMethod>,
    pub moneyDigits: Option<u32>,
    pub trailingStopLoss: Option<bool>,
}

impl<'a> MessageRead<'a> for ProtoOAPosition<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.positionId = r.read_int64(bytes)?,
                Ok(18) => msg.tradeData = r.read_message::<spotware::ProtoOATradeData>(bytes)?,
                Ok(24) => msg.positionStatus = r.read_enum(bytes)?,
                Ok(32) => msg.swap = r.read_int64(bytes)?,
                Ok(41) => msg.price = Some(r.read_double(bytes)?),
                Ok(49) => msg.stopLoss = Some(r.read_double(bytes)?),
                Ok(57) => msg.takeProfit = Some(r.read_double(bytes)?),
                Ok(64) => msg.utcLastUpdateTimestamp = Some(r.read_int64(bytes)?),
                Ok(72) => msg.commission = Some(r.read_int64(bytes)?),
                Ok(81) => msg.marginRate = Some(r.read_double(bytes)?),
                Ok(88) => msg.mirroringCommission = Some(r.read_int64(bytes)?),
                Ok(96) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(104) => msg.usedMargin = Some(r.read_uint64(bytes)?),
                Ok(112) => msg.stopLossTriggerMethod = Some(r.read_enum(bytes)?),
                Ok(120) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(128) => msg.trailingStopLoss = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAPosition<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_len((self.tradeData).get_size())
        + 1 + sizeof_varint(*(&self.positionStatus) as u64)
        + 1 + sizeof_varint(*(&self.swap) as u64)
        + self.price.as_ref().map_or(0, |&m| 1 + 8)
        + self.stopLoss.as_ref().map_or(0, |&m| 1 + 8)
        + self.takeProfit.as_ref().map_or(0, |&m| 1 + 8)
        + self.utcLastUpdateTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.commission.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.marginRate.as_ref().map_or(0, |&m| 1 + 8)
        + self.mirroringCommission.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.usedMargin.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stopLossTriggerMethod.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.trailingStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.positionId))?;
        w.write_with_tag(18, |w| w.write_message(&self.tradeData))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.positionStatus as i32))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.swap))?;
        self.price.as_ref().map_or(Ok(()), |&m| w.write_with_tag(41, |w| w.write_double(*&m)))?;
        self.stopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(49, |w| w.write_double(*&m)))?;
        self.takeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(57, |w| w.write_double(*&m)))?;
        self.utcLastUpdateTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int64(*&m)))?;
        self.commission.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_int64(*&m)))?;
        self.marginRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(81, |w| w.write_double(*&m)))?;
        self.mirroringCommission.as_ref().map_or(Ok(()), |&m| w.write_with_tag(88, |w| w.write_int64(*&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(96, |w| w.write_bool(*&m)))?;
        self.usedMargin.as_ref().map_or(Ok(()), |&m| w.write_with_tag(104, |w| w.write_uint64(*&m)))?;
        self.stopLossTriggerMethod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(112, |w| w.write_enum(*&m as i32)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(120, |w| w.write_uint32(*&m)))?;
        self.trailingStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(128, |w| w.write_bool(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrder<'a> {
    pub orderId: i64,
    pub tradeData: spotware::ProtoOATradeData<'a>,
    pub orderType: spotware::ProtoOAOrderType,
    pub orderStatus: spotware::ProtoOAOrderStatus,
    pub expirationTimestamp: Option<i64>,
    pub executionPrice: Option<f64>,
    pub executedVolume: Option<i64>,
    pub utcLastUpdateTimestamp: Option<i64>,
    pub baseSlippagePrice: Option<f64>,
    pub slippageInPoints: Option<i64>,
    pub closingOrder: Option<bool>,
    pub limitPrice: Option<f64>,
    pub stopPrice: Option<f64>,
    pub stopLoss: Option<f64>,
    pub takeProfit: Option<f64>,
    pub clientOrderId: Option<Cow<'a, str>>,
    pub timeInForce: Option<spotware::ProtoOATimeInForce>,
    pub positionId: Option<i64>,
    pub relativeStopLoss: Option<i64>,
    pub relativeTakeProfit: Option<i64>,
    pub isStopOut: Option<bool>,
    pub trailingStopLoss: Option<bool>,
    pub stopTriggerMethod: Option<spotware::ProtoOAOrderTriggerMethod>,
}

impl<'a> MessageRead<'a> for ProtoOAOrder<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.orderId = r.read_int64(bytes)?,
                Ok(18) => msg.tradeData = r.read_message::<spotware::ProtoOATradeData>(bytes)?,
                Ok(24) => msg.orderType = r.read_enum(bytes)?,
                Ok(32) => msg.orderStatus = r.read_enum(bytes)?,
                Ok(48) => msg.expirationTimestamp = Some(r.read_int64(bytes)?),
                Ok(57) => msg.executionPrice = Some(r.read_double(bytes)?),
                Ok(64) => msg.executedVolume = Some(r.read_int64(bytes)?),
                Ok(72) => msg.utcLastUpdateTimestamp = Some(r.read_int64(bytes)?),
                Ok(81) => msg.baseSlippagePrice = Some(r.read_double(bytes)?),
                Ok(88) => msg.slippageInPoints = Some(r.read_int64(bytes)?),
                Ok(96) => msg.closingOrder = Some(r.read_bool(bytes)?),
                Ok(105) => msg.limitPrice = Some(r.read_double(bytes)?),
                Ok(113) => msg.stopPrice = Some(r.read_double(bytes)?),
                Ok(121) => msg.stopLoss = Some(r.read_double(bytes)?),
                Ok(129) => msg.takeProfit = Some(r.read_double(bytes)?),
                Ok(138) => msg.clientOrderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(144) => msg.timeInForce = Some(r.read_enum(bytes)?),
                Ok(152) => msg.positionId = Some(r.read_int64(bytes)?),
                Ok(160) => msg.relativeStopLoss = Some(r.read_int64(bytes)?),
                Ok(168) => msg.relativeTakeProfit = Some(r.read_int64(bytes)?),
                Ok(176) => msg.isStopOut = Some(r.read_bool(bytes)?),
                Ok(184) => msg.trailingStopLoss = Some(r.read_bool(bytes)?),
                Ok(192) => msg.stopTriggerMethod = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAOrder<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.orderId) as u64)
        + 1 + sizeof_len((self.tradeData).get_size())
        + 1 + sizeof_varint(*(&self.orderType) as u64)
        + 1 + sizeof_varint(*(&self.orderStatus) as u64)
        + self.expirationTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.executionPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.executedVolume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.utcLastUpdateTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.baseSlippagePrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.slippageInPoints.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.closingOrder.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.limitPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.stopPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.stopLoss.as_ref().map_or(0, |&m| 1 + 8)
        + self.takeProfit.as_ref().map_or(0, |&m| 2 + 8)
        + self.clientOrderId.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.timeInForce.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.positionId.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.relativeStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.relativeTakeProfit.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.isStopOut.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.trailingStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.stopTriggerMethod.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.orderId))?;
        w.write_with_tag(18, |w| w.write_message(&self.tradeData))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.orderType as i32))?;
        w.write_with_tag(32, |w| w.write_enum(*&self.orderStatus as i32))?;
        self.expirationTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        self.executionPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(57, |w| w.write_double(*&m)))?;
        self.executedVolume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int64(*&m)))?;
        self.utcLastUpdateTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_int64(*&m)))?;
        self.baseSlippagePrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(81, |w| w.write_double(*&m)))?;
        self.slippageInPoints.as_ref().map_or(Ok(()), |&m| w.write_with_tag(88, |w| w.write_int64(*&m)))?;
        self.closingOrder.as_ref().map_or(Ok(()), |&m| w.write_with_tag(96, |w| w.write_bool(*&m)))?;
        self.limitPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(105, |w| w.write_double(*&m)))?;
        self.stopPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(113, |w| w.write_double(*&m)))?;
        self.stopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(121, |w| w.write_double(*&m)))?;
        self.takeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(129, |w| w.write_double(*&m)))?;
        self.clientOrderId.as_ref().map_or(Ok(()), |m| w.write_with_tag(138, |w| w.write_string(&m)))?;
        self.timeInForce.as_ref().map_or(Ok(()), |&m| w.write_with_tag(144, |w| w.write_enum(*&m as i32)))?;
        self.positionId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(152, |w| w.write_int64(*&m)))?;
        self.relativeStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(160, |w| w.write_int64(*&m)))?;
        self.relativeTakeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(168, |w| w.write_int64(*&m)))?;
        self.isStopOut.as_ref().map_or(Ok(()), |&m| w.write_with_tag(176, |w| w.write_bool(*&m)))?;
        self.trailingStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(184, |w| w.write_bool(*&m)))?;
        self.stopTriggerMethod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(192, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOABonusDepositWithdraw<'a> {
    pub operationType: spotware::ProtoOAChangeBonusType,
    pub bonusHistoryId: i64,
    pub managerBonus: i64,
    pub managerDelta: i64,
    pub ibBonus: i64,
    pub ibDelta: i64,
    pub changeBonusTimestamp: i64,
    pub externalNote: Option<Cow<'a, str>>,
    pub introducingBrokerId: Option<i64>,
    pub moneyDigits: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOABonusDepositWithdraw<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.operationType = r.read_enum(bytes)?,
                Ok(16) => msg.bonusHistoryId = r.read_int64(bytes)?,
                Ok(24) => msg.managerBonus = r.read_int64(bytes)?,
                Ok(32) => msg.managerDelta = r.read_int64(bytes)?,
                Ok(40) => msg.ibBonus = r.read_int64(bytes)?,
                Ok(48) => msg.ibDelta = r.read_int64(bytes)?,
                Ok(56) => msg.changeBonusTimestamp = r.read_int64(bytes)?,
                Ok(66) => msg.externalNote = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(72) => msg.introducingBrokerId = Some(r.read_int64(bytes)?),
                Ok(80) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOABonusDepositWithdraw<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.operationType) as u64)
        + 1 + sizeof_varint(*(&self.bonusHistoryId) as u64)
        + 1 + sizeof_varint(*(&self.managerBonus) as u64)
        + 1 + sizeof_varint(*(&self.managerDelta) as u64)
        + 1 + sizeof_varint(*(&self.ibBonus) as u64)
        + 1 + sizeof_varint(*(&self.ibDelta) as u64)
        + 1 + sizeof_varint(*(&self.changeBonusTimestamp) as u64)
        + self.externalNote.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.introducingBrokerId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.operationType as i32))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.bonusHistoryId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.managerBonus))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.managerDelta))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.ibBonus))?;
        w.write_with_tag(48, |w| w.write_int64(*&self.ibDelta))?;
        w.write_with_tag(56, |w| w.write_int64(*&self.changeBonusTimestamp))?;
        self.externalNote.as_ref().map_or(Ok(()), |m| w.write_with_tag(66, |w| w.write_string(&m)))?;
        self.introducingBrokerId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_int64(*&m)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADepositWithdraw<'a> {
    pub operationType: spotware::ProtoOAChangeBalanceType,
    pub balanceHistoryId: i64,
    pub balance: i64,
    pub delta: i64,
    pub changeBalanceTimestamp: i64,
    pub externalNote: Option<Cow<'a, str>>,
    pub balanceVersion: Option<i64>,
    pub equity: Option<i64>,
    pub moneyDigits: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOADepositWithdraw<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.operationType = r.read_enum(bytes)?,
                Ok(16) => msg.balanceHistoryId = r.read_int64(bytes)?,
                Ok(24) => msg.balance = r.read_int64(bytes)?,
                Ok(32) => msg.delta = r.read_int64(bytes)?,
                Ok(40) => msg.changeBalanceTimestamp = r.read_int64(bytes)?,
                Ok(50) => msg.externalNote = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.balanceVersion = Some(r.read_int64(bytes)?),
                Ok(64) => msg.equity = Some(r.read_int64(bytes)?),
                Ok(72) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOADepositWithdraw<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.operationType) as u64)
        + 1 + sizeof_varint(*(&self.balanceHistoryId) as u64)
        + 1 + sizeof_varint(*(&self.balance) as u64)
        + 1 + sizeof_varint(*(&self.delta) as u64)
        + 1 + sizeof_varint(*(&self.changeBalanceTimestamp) as u64)
        + self.externalNote.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.balanceVersion.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.equity.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.operationType as i32))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.balanceHistoryId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.balance))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.delta))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.changeBalanceTimestamp))?;
        self.externalNote.as_ref().map_or(Ok(()), |m| w.write_with_tag(50, |w| w.write_string(&m)))?;
        self.balanceVersion.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_int64(*&m)))?;
        self.equity.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int64(*&m)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADeal {
    pub dealId: i64,
    pub orderId: i64,
    pub positionId: i64,
    pub volume: i64,
    pub filledVolume: i64,
    pub symbolId: i64,
    pub createTimestamp: i64,
    pub executionTimestamp: i64,
    pub utcLastUpdateTimestamp: Option<i64>,
    pub executionPrice: Option<f64>,
    pub tradeSide: spotware::ProtoOATradeSide,
    pub dealStatus: spotware::ProtoOADealStatus,
    pub marginRate: Option<f64>,
    pub commission: Option<i64>,
    pub baseToUsdConversionRate: Option<f64>,
    pub closePositionDetail: Option<spotware::ProtoOAClosePositionDetail>,
    pub moneyDigits: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOADeal {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.dealId = r.read_int64(bytes)?,
                Ok(16) => msg.orderId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(32) => msg.volume = r.read_int64(bytes)?,
                Ok(40) => msg.filledVolume = r.read_int64(bytes)?,
                Ok(48) => msg.symbolId = r.read_int64(bytes)?,
                Ok(56) => msg.createTimestamp = r.read_int64(bytes)?,
                Ok(64) => msg.executionTimestamp = r.read_int64(bytes)?,
                Ok(72) => msg.utcLastUpdateTimestamp = Some(r.read_int64(bytes)?),
                Ok(81) => msg.executionPrice = Some(r.read_double(bytes)?),
                Ok(88) => msg.tradeSide = r.read_enum(bytes)?,
                Ok(96) => msg.dealStatus = r.read_enum(bytes)?,
                Ok(105) => msg.marginRate = Some(r.read_double(bytes)?),
                Ok(112) => msg.commission = Some(r.read_int64(bytes)?),
                Ok(121) => msg.baseToUsdConversionRate = Some(r.read_double(bytes)?),
                Ok(130) => msg.closePositionDetail = Some(r.read_message::<spotware::ProtoOAClosePositionDetail>(bytes)?),
                Ok(136) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADeal {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.dealId) as u64)
        + 1 + sizeof_varint(*(&self.orderId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_varint(*(&self.volume) as u64)
        + 1 + sizeof_varint(*(&self.filledVolume) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_varint(*(&self.createTimestamp) as u64)
        + 1 + sizeof_varint(*(&self.executionTimestamp) as u64)
        + self.utcLastUpdateTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.executionPrice.as_ref().map_or(0, |&m| 1 + 8)
        + 1 + sizeof_varint(*(&self.tradeSide) as u64)
        + 1 + sizeof_varint(*(&self.dealStatus) as u64)
        + self.marginRate.as_ref().map_or(0, |&m| 1 + 8)
        + self.commission.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.baseToUsdConversionRate.as_ref().map_or(0, |&m| 1 + 8)
        + self.closePositionDetail.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.moneyDigits.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.dealId))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.orderId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.volume))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.filledVolume))?;
        w.write_with_tag(48, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(56, |w| w.write_int64(*&self.createTimestamp))?;
        w.write_with_tag(64, |w| w.write_int64(*&self.executionTimestamp))?;
        self.utcLastUpdateTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_int64(*&m)))?;
        self.executionPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(81, |w| w.write_double(*&m)))?;
        w.write_with_tag(88, |w| w.write_enum(*&self.tradeSide as i32))?;
        w.write_with_tag(96, |w| w.write_enum(*&self.dealStatus as i32))?;
        self.marginRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(105, |w| w.write_double(*&m)))?;
        self.commission.as_ref().map_or(Ok(()), |&m| w.write_with_tag(112, |w| w.write_int64(*&m)))?;
        self.baseToUsdConversionRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(121, |w| w.write_double(*&m)))?;
        self.closePositionDetail.as_ref().map_or(Ok(()), |m| w.write_with_tag(130, |w| w.write_message(m)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(136, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealOffset {
    pub dealId: i64,
    pub volume: i64,
    pub executionTimestamp: Option<i64>,
    pub executionPrice: Option<f64>,
}

impl<'a> MessageRead<'a> for ProtoOADealOffset {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.dealId = r.read_int64(bytes)?,
                Ok(16) => msg.volume = r.read_int64(bytes)?,
                Ok(24) => msg.executionTimestamp = Some(r.read_int64(bytes)?),
                Ok(33) => msg.executionPrice = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealOffset {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.dealId) as u64)
        + 1 + sizeof_varint(*(&self.volume) as u64)
        + self.executionTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.executionPrice.as_ref().map_or(0, |&m| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.dealId))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.volume))?;
        self.executionTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.executionPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(33, |w| w.write_double(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAClosePositionDetail {
    pub entryPrice: f64,
    pub grossProfit: i64,
    pub swap: i64,
    pub commission: i64,
    pub balance: i64,
    pub quoteToDepositConversionRate: Option<f64>,
    pub closedVolume: Option<i64>,
    pub balanceVersion: Option<i64>,
    pub moneyDigits: Option<u32>,
    pub pnlConversionFee: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAClosePositionDetail {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.entryPrice = r.read_double(bytes)?,
                Ok(16) => msg.grossProfit = r.read_int64(bytes)?,
                Ok(24) => msg.swap = r.read_int64(bytes)?,
                Ok(32) => msg.commission = r.read_int64(bytes)?,
                Ok(40) => msg.balance = r.read_int64(bytes)?,
                Ok(49) => msg.quoteToDepositConversionRate = Some(r.read_double(bytes)?),
                Ok(56) => msg.closedVolume = Some(r.read_int64(bytes)?),
                Ok(64) => msg.balanceVersion = Some(r.read_int64(bytes)?),
                Ok(72) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.pnlConversionFee = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAClosePositionDetail {
    fn get_size(&self) -> usize {
        1 + 8
        + 1 + sizeof_varint(*(&self.grossProfit) as u64)
        + 1 + sizeof_varint(*(&self.swap) as u64)
        + 1 + sizeof_varint(*(&self.commission) as u64)
        + 1 + sizeof_varint(*(&self.balance) as u64)
        + self.quoteToDepositConversionRate.as_ref().map_or(0, |&m| 1 + 8)
        + self.closedVolume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.balanceVersion.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.pnlConversionFee.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(9, |w| w.write_double(*&self.entryPrice))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.grossProfit))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.swap))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.commission))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.balance))?;
        self.quoteToDepositConversionRate.as_ref().map_or(Ok(()), |&m| w.write_with_tag(49, |w| w.write_double(*&m)))?;
        self.closedVolume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_int64(*&m)))?;
        self.balanceVersion.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int64(*&m)))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_uint32(*&m)))?;
        self.pnlConversionFee.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATrendbar {
    pub volume: i64,
    pub period: Option<spotware::ProtoOATrendbarPeriod>,
    pub low: Option<i64>,
    pub deltaOpen: Option<u64>,
    pub deltaClose: Option<u64>,
    pub deltaHigh: Option<u64>,
    pub utcTimestampInMinutes: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOATrendbar {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(24) => msg.volume = r.read_int64(bytes)?,
                Ok(32) => msg.period = Some(r.read_enum(bytes)?),
                Ok(40) => msg.low = Some(r.read_int64(bytes)?),
                Ok(48) => msg.deltaOpen = Some(r.read_uint64(bytes)?),
                Ok(56) => msg.deltaClose = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.deltaHigh = Some(r.read_uint64(bytes)?),
                Ok(72) => msg.utcTimestampInMinutes = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOATrendbar {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.volume) as u64)
        + self.period.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.low.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.deltaOpen.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.deltaClose.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.deltaHigh.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.utcTimestampInMinutes.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(24, |w| w.write_int64(*&self.volume))?;
        self.period.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_enum(*&m as i32)))?;
        self.low.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.deltaOpen.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_uint64(*&m)))?;
        self.deltaClose.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_uint64(*&m)))?;
        self.deltaHigh.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_uint64(*&m)))?;
        self.utcTimestampInMinutes.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAExpectedMargin {
    pub volume: i64,
    pub buyMargin: i64,
    pub sellMargin: i64,
}

impl<'a> MessageRead<'a> for ProtoOAExpectedMargin {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.volume = r.read_int64(bytes)?,
                Ok(16) => msg.buyMargin = r.read_int64(bytes)?,
                Ok(24) => msg.sellMargin = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAExpectedMargin {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.volume) as u64)
        + 1 + sizeof_varint(*(&self.buyMargin) as u64)
        + 1 + sizeof_varint(*(&self.sellMargin) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.volume))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.buyMargin))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.sellMargin))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATickData {
    pub timestamp: i64,
    pub tick: i64,
}

impl<'a> MessageRead<'a> for ProtoOATickData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.timestamp = r.read_int64(bytes)?,
                Ok(16) => msg.tick = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOATickData {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.timestamp) as u64)
        + 1 + sizeof_varint(*(&self.tick) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.timestamp))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.tick))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOACtidProfile {
    pub userId: i64,
}

impl<'a> MessageRead<'a> for ProtoOACtidProfile {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.userId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOACtidProfile {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.userId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.userId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOACtidTraderAccount<'a> {
    pub ctidTraderAccountId: i64,
    pub isLive: Option<bool>,
    pub traderLogin: Option<i64>,
    pub lastClosingDealTimestamp: Option<i64>,
    pub lastBalanceUpdateTimestamp: Option<i64>,
    pub brokerTitleShort: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOACtidTraderAccount<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(16) => msg.isLive = Some(r.read_bool(bytes)?),
                Ok(24) => msg.traderLogin = Some(r.read_int64(bytes)?),
                Ok(32) => msg.lastClosingDealTimestamp = Some(r.read_int64(bytes)?),
                Ok(40) => msg.lastBalanceUpdateTimestamp = Some(r.read_int64(bytes)?),
                Ok(50) => msg.brokerTitleShort = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOACtidTraderAccount<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.isLive.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.traderLogin.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.lastClosingDealTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.lastBalanceUpdateTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.brokerTitleShort.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.isLive.as_ref().map_or(Ok(()), |&m| w.write_with_tag(16, |w| w.write_bool(*&m)))?;
        self.traderLogin.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.lastClosingDealTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.lastBalanceUpdateTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.brokerTitleShort.as_ref().map_or(Ok(()), |m| w.write_with_tag(50, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAssetClass<'a> {
    pub id: Option<i64>,
    pub name: Option<Cow<'a, str>>,
    pub sortingNumber: Option<f64>,
}

impl<'a> MessageRead<'a> for ProtoOAAssetClass<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = Some(r.read_int64(bytes)?),
                Ok(18) => msg.name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(25) => msg.sortingNumber = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAssetClass<'a> {
    fn get_size(&self) -> usize {
        self.id.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.sortingNumber.as_ref().map_or(0, |&m| 1 + 8)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.id.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_int64(*&m)))?;
        self.name.as_ref().map_or(Ok(()), |m| w.write_with_tag(18, |w| w.write_string(&m)))?;
        self.sortingNumber.as_ref().map_or(Ok(()), |&m| w.write_with_tag(25, |w| w.write_double(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADepthQuote {
    pub id: u64,
    pub size: u64,
    pub bid: Option<u64>,
    pub ask: Option<u64>,
}

impl<'a> MessageRead<'a> for ProtoOADepthQuote {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint64(bytes)?,
                Ok(24) => msg.size = r.read_uint64(bytes)?,
                Ok(32) => msg.bid = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.ask = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADepthQuote {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_varint(*(&self.size) as u64)
        + self.bid.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ask.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint64(*&self.id))?;
        w.write_with_tag(24, |w| w.write_uint64(*&self.size))?;
        self.bid.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_uint64(*&m)))?;
        self.ask.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_uint64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCall {
    pub marginCallType: spotware::ProtoOANotificationType,
    pub marginLevelThreshold: f64,
    pub utcLastUpdateTimestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCall {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.marginCallType = r.read_enum(bytes)?,
                Ok(17) => msg.marginLevelThreshold = r.read_double(bytes)?,
                Ok(24) => msg.utcLastUpdateTimestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCall {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.marginCallType) as u64)
        + 1 + 8
        + self.utcLastUpdateTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_enum(*&self.marginCallType as i32))?;
        w.write_with_tag(17, |w| w.write_double(*&self.marginLevelThreshold))?;
        self.utcLastUpdateTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAApplicationAuthReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub clientId: Cow<'a, str>,
    pub clientSecret: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOAApplicationAuthReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.clientId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.clientSecret = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAApplicationAuthReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.clientId).len())
        + 1 + sizeof_len((&self.clientSecret).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.clientId))?;
        w.write_with_tag(26, |w| w.write_string(&self.clientSecret))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAApplicationAuthRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
}

impl<'a> MessageRead<'a> for ProtoOAApplicationAuthRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAApplicationAuthRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountAuthReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub accessToken: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOAAccountAuthReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.accessToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAccountAuthReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((&self.accessToken).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_string(&self.accessToken))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountAuthRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAccountAuthRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAccountAuthRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAErrorRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: Option<i64>,
    pub errorCode: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub maintenanceEndTimestamp: Option<i64>,
    pub retryAfter: Option<u64>,
}

impl<'a> MessageRead<'a> for ProtoOAErrorRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = Some(r.read_int64(bytes)?),
                Ok(26) => msg.errorCode = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(40) => msg.maintenanceEndTimestamp = Some(r.read_int64(bytes)?),
                Ok(48) => msg.retryAfter = Some(r.read_uint64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAErrorRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ctidTraderAccountId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.errorCode).len())
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.maintenanceEndTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.retryAfter.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        self.ctidTraderAccountId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(16, |w| w.write_int64(*&m)))?;
        w.write_with_tag(26, |w| w.write_string(&self.errorCode))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(34, |w| w.write_string(&m)))?;
        self.maintenanceEndTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.retryAfter.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_uint64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAClientDisconnectEvent<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub reason: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOAClientDisconnectEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.reason = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAClientDisconnectEvent<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.reason.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        self.reason.as_ref().map_or(Ok(()), |m| w.write_with_tag(18, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountsTokenInvalidatedEvent<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountIds: Vec<i64>,
    pub reason: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOAAccountsTokenInvalidatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountIds.push(r.read_int64(bytes)?),
                Ok(26) => msg.reason = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAccountsTokenInvalidatedEvent<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ctidTraderAccountIds.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
        + self.reason.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        for s in &self.ctidTraderAccountIds { w.write_with_tag(16, |w| w.write_int64(*&*s))?; }
        self.reason.as_ref().map_or(Ok(()), |m| w.write_with_tag(26, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAVersionReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
}

impl<'a> MessageRead<'a> for ProtoOAVersionReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAVersionReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAVersionRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub version: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOAVersionRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.version = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAVersionRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.version).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.version))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOANewOrderReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: i64,
    pub orderType: spotware::ProtoOAOrderType,
    pub tradeSide: spotware::ProtoOATradeSide,
    pub volume: i64,
    pub limitPrice: Option<f64>,
    pub stopPrice: Option<f64>,
    pub timeInForce: Option<spotware::ProtoOATimeInForce>,
    pub expirationTimestamp: Option<i64>,
    pub stopLoss: Option<f64>,
    pub takeProfit: Option<f64>,
    pub comment: Option<Cow<'a, str>>,
    pub baseSlippagePrice: Option<f64>,
    pub slippageInPoints: Option<i32>,
    pub label: Option<Cow<'a, str>>,
    pub positionId: Option<i64>,
    pub clientOrderId: Option<Cow<'a, str>>,
    pub relativeStopLoss: Option<i64>,
    pub relativeTakeProfit: Option<i64>,
    pub guaranteedStopLoss: Option<bool>,
    pub trailingStopLoss: Option<bool>,
    pub stopTriggerMethod: Option<spotware::ProtoOAOrderTriggerMethod>,
}

impl<'a> MessageRead<'a> for ProtoOANewOrderReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId = r.read_int64(bytes)?,
                Ok(32) => msg.orderType = r.read_enum(bytes)?,
                Ok(40) => msg.tradeSide = r.read_enum(bytes)?,
                Ok(48) => msg.volume = r.read_int64(bytes)?,
                Ok(57) => msg.limitPrice = Some(r.read_double(bytes)?),
                Ok(65) => msg.stopPrice = Some(r.read_double(bytes)?),
                Ok(72) => msg.timeInForce = Some(r.read_enum(bytes)?),
                Ok(80) => msg.expirationTimestamp = Some(r.read_int64(bytes)?),
                Ok(89) => msg.stopLoss = Some(r.read_double(bytes)?),
                Ok(97) => msg.takeProfit = Some(r.read_double(bytes)?),
                Ok(106) => msg.comment = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(113) => msg.baseSlippagePrice = Some(r.read_double(bytes)?),
                Ok(120) => msg.slippageInPoints = Some(r.read_int32(bytes)?),
                Ok(130) => msg.label = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(136) => msg.positionId = Some(r.read_int64(bytes)?),
                Ok(146) => msg.clientOrderId = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(152) => msg.relativeStopLoss = Some(r.read_int64(bytes)?),
                Ok(160) => msg.relativeTakeProfit = Some(r.read_int64(bytes)?),
                Ok(168) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(176) => msg.trailingStopLoss = Some(r.read_bool(bytes)?),
                Ok(184) => msg.stopTriggerMethod = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOANewOrderReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_varint(*(&self.orderType) as u64)
        + 1 + sizeof_varint(*(&self.tradeSide) as u64)
        + 1 + sizeof_varint(*(&self.volume) as u64)
        + self.limitPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.stopPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.timeInForce.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.expirationTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stopLoss.as_ref().map_or(0, |&m| 1 + 8)
        + self.takeProfit.as_ref().map_or(0, |&m| 1 + 8)
        + self.comment.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.baseSlippagePrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.slippageInPoints.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.label.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.positionId.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.clientOrderId.as_ref().map_or(0, |m| 2 + sizeof_len((&m).len()))
        + self.relativeStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.relativeTakeProfit.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.trailingStopLoss.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
        + self.stopTriggerMethod.as_ref().map_or(0, |&m| 2 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(32, |w| w.write_enum(*&self.orderType as i32))?;
        w.write_with_tag(40, |w| w.write_enum(*&self.tradeSide as i32))?;
        w.write_with_tag(48, |w| w.write_int64(*&self.volume))?;
        self.limitPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(57, |w| w.write_double(*&m)))?;
        self.stopPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(65, |w| w.write_double(*&m)))?;
        self.timeInForce.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_enum(*&m as i32)))?;
        self.expirationTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_int64(*&m)))?;
        self.stopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(89, |w| w.write_double(*&m)))?;
        self.takeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(97, |w| w.write_double(*&m)))?;
        self.comment.as_ref().map_or(Ok(()), |m| w.write_with_tag(106, |w| w.write_string(&m)))?;
        self.baseSlippagePrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(113, |w| w.write_double(*&m)))?;
        self.slippageInPoints.as_ref().map_or(Ok(()), |&m| w.write_with_tag(120, |w| w.write_int32(*&m)))?;
        self.label.as_ref().map_or(Ok(()), |m| w.write_with_tag(130, |w| w.write_string(&m)))?;
        self.positionId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(136, |w| w.write_int64(*&m)))?;
        self.clientOrderId.as_ref().map_or(Ok(()), |m| w.write_with_tag(146, |w| w.write_string(&m)))?;
        self.relativeStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(152, |w| w.write_int64(*&m)))?;
        self.relativeTakeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(160, |w| w.write_int64(*&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(168, |w| w.write_bool(*&m)))?;
        self.trailingStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(176, |w| w.write_bool(*&m)))?;
        self.stopTriggerMethod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(184, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAExecutionEvent<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub executionType: spotware::ProtoOAExecutionType,
    pub position: Option<spotware::ProtoOAPosition<'a>>,
    pub order: Option<spotware::ProtoOAOrder<'a>>,
    pub deal: Option<spotware::ProtoOADeal>,
    pub bonusDepositWithdraw: Option<spotware::ProtoOABonusDepositWithdraw<'a>>,
    pub depositWithdraw: Option<spotware::ProtoOADepositWithdraw<'a>>,
    pub errorCode: Option<Cow<'a, str>>,
    pub isServerEvent: Option<bool>,
}

impl<'a> MessageRead<'a> for ProtoOAExecutionEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.executionType = r.read_enum(bytes)?,
                Ok(34) => msg.position = Some(r.read_message::<spotware::ProtoOAPosition>(bytes)?),
                Ok(42) => msg.order = Some(r.read_message::<spotware::ProtoOAOrder>(bytes)?),
                Ok(50) => msg.deal = Some(r.read_message::<spotware::ProtoOADeal>(bytes)?),
                Ok(58) => msg.bonusDepositWithdraw = Some(r.read_message::<spotware::ProtoOABonusDepositWithdraw>(bytes)?),
                Ok(66) => msg.depositWithdraw = Some(r.read_message::<spotware::ProtoOADepositWithdraw>(bytes)?),
                Ok(74) => msg.errorCode = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(80) => msg.isServerEvent = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAExecutionEvent<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.executionType) as u64)
        + self.position.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.order.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.deal.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.bonusDepositWithdraw.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.depositWithdraw.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.errorCode.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + self.isServerEvent.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.executionType as i32))?;
        self.position.as_ref().map_or(Ok(()), |m| w.write_with_tag(34, |w| w.write_message(m)))?;
        self.order.as_ref().map_or(Ok(()), |m| w.write_with_tag(42, |w| w.write_message(m)))?;
        self.deal.as_ref().map_or(Ok(()), |m| w.write_with_tag(50, |w| w.write_message(m)))?;
        self.bonusDepositWithdraw.as_ref().map_or(Ok(()), |m| w.write_with_tag(58, |w| w.write_message(m)))?;
        self.depositWithdraw.as_ref().map_or(Ok(()), |m| w.write_with_tag(66, |w| w.write_message(m)))?;
        self.errorCode.as_ref().map_or(Ok(()), |m| w.write_with_tag(74, |w| w.write_string(&m)))?;
        self.isServerEvent.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_bool(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOACancelOrderReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub orderId: i64,
}

impl<'a> MessageRead<'a> for ProtoOACancelOrderReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.orderId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOACancelOrderReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.orderId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.orderId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAmendOrderReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub orderId: i64,
    pub volume: Option<i64>,
    pub limitPrice: Option<f64>,
    pub stopPrice: Option<f64>,
    pub expirationTimestamp: Option<i64>,
    pub stopLoss: Option<f64>,
    pub takeProfit: Option<f64>,
    pub slippageInPoints: Option<i32>,
    pub relativeStopLoss: Option<i64>,
    pub relativeTakeProfit: Option<i64>,
    pub guaranteedStopLoss: Option<bool>,
    pub trailingStopLoss: Option<bool>,
    pub stopTriggerMethod: Option<spotware::ProtoOAOrderTriggerMethod>,
}

impl<'a> MessageRead<'a> for ProtoOAAmendOrderReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.orderId = r.read_int64(bytes)?,
                Ok(32) => msg.volume = Some(r.read_int64(bytes)?),
                Ok(41) => msg.limitPrice = Some(r.read_double(bytes)?),
                Ok(49) => msg.stopPrice = Some(r.read_double(bytes)?),
                Ok(56) => msg.expirationTimestamp = Some(r.read_int64(bytes)?),
                Ok(65) => msg.stopLoss = Some(r.read_double(bytes)?),
                Ok(73) => msg.takeProfit = Some(r.read_double(bytes)?),
                Ok(80) => msg.slippageInPoints = Some(r.read_int32(bytes)?),
                Ok(88) => msg.relativeStopLoss = Some(r.read_int64(bytes)?),
                Ok(96) => msg.relativeTakeProfit = Some(r.read_int64(bytes)?),
                Ok(104) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(112) => msg.trailingStopLoss = Some(r.read_bool(bytes)?),
                Ok(120) => msg.stopTriggerMethod = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAmendOrderReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.orderId) as u64)
        + self.volume.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.limitPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.stopPrice.as_ref().map_or(0, |&m| 1 + 8)
        + self.expirationTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stopLoss.as_ref().map_or(0, |&m| 1 + 8)
        + self.takeProfit.as_ref().map_or(0, |&m| 1 + 8)
        + self.slippageInPoints.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.relativeStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.relativeTakeProfit.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.trailingStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stopTriggerMethod.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.orderId))?;
        self.volume.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.limitPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(41, |w| w.write_double(*&m)))?;
        self.stopPrice.as_ref().map_or(Ok(()), |&m| w.write_with_tag(49, |w| w.write_double(*&m)))?;
        self.expirationTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_int64(*&m)))?;
        self.stopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(65, |w| w.write_double(*&m)))?;
        self.takeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(73, |w| w.write_double(*&m)))?;
        self.slippageInPoints.as_ref().map_or(Ok(()), |&m| w.write_with_tag(80, |w| w.write_int32(*&m)))?;
        self.relativeStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(88, |w| w.write_int64(*&m)))?;
        self.relativeTakeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(96, |w| w.write_int64(*&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(104, |w| w.write_bool(*&m)))?;
        self.trailingStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(112, |w| w.write_bool(*&m)))?;
        self.stopTriggerMethod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(120, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAmendPositionSLTPReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: i64,
    pub stopLoss: Option<f64>,
    pub takeProfit: Option<f64>,
    pub guaranteedStopLoss: Option<bool>,
    pub trailingStopLoss: Option<bool>,
    pub stopLossTriggerMethod: Option<spotware::ProtoOAOrderTriggerMethod>,
}

impl<'a> MessageRead<'a> for ProtoOAAmendPositionSLTPReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(33) => msg.stopLoss = Some(r.read_double(bytes)?),
                Ok(41) => msg.takeProfit = Some(r.read_double(bytes)?),
                Ok(56) => msg.guaranteedStopLoss = Some(r.read_bool(bytes)?),
                Ok(64) => msg.trailingStopLoss = Some(r.read_bool(bytes)?),
                Ok(72) => msg.stopLossTriggerMethod = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAmendPositionSLTPReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + self.stopLoss.as_ref().map_or(0, |&m| 1 + 8)
        + self.takeProfit.as_ref().map_or(0, |&m| 1 + 8)
        + self.guaranteedStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.trailingStopLoss.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.stopLossTriggerMethod.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        self.stopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(33, |w| w.write_double(*&m)))?;
        self.takeProfit.as_ref().map_or(Ok(()), |&m| w.write_with_tag(41, |w| w.write_double(*&m)))?;
        self.guaranteedStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_bool(*&m)))?;
        self.trailingStopLoss.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_bool(*&m)))?;
        self.stopLossTriggerMethod.as_ref().map_or(Ok(()), |&m| w.write_with_tag(72, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAClosePositionReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: i64,
    pub volume: i64,
}

impl<'a> MessageRead<'a> for ProtoOAClosePositionReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(32) => msg.volume = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAClosePositionReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_varint(*(&self.volume) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.volume))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATrailingSLChangedEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: i64,
    pub orderId: i64,
    pub stopPrice: f64,
    pub utcLastUpdateTimestamp: i64,
}

impl<'a> MessageRead<'a> for ProtoOATrailingSLChangedEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(32) => msg.orderId = r.read_int64(bytes)?,
                Ok(41) => msg.stopPrice = r.read_double(bytes)?,
                Ok(48) => msg.utcLastUpdateTimestamp = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOATrailingSLChangedEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_varint(*(&self.orderId) as u64)
        + 1 + 8
        + 1 + sizeof_varint(*(&self.utcLastUpdateTimestamp) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.orderId))?;
        w.write_with_tag(41, |w| w.write_double(*&self.stopPrice))?;
        w.write_with_tag(48, |w| w.write_int64(*&self.utcLastUpdateTimestamp))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAssetListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAssetListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAssetListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAssetListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub asset: Vec<spotware::ProtoOAAsset<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOAAssetListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.asset.push(r.read_message::<spotware::ProtoOAAsset>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAssetListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.asset.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.asset { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolsListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub includeArchivedSymbols: Option<bool>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolsListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.includeArchivedSymbols = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASymbolsListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.includeArchivedSymbols.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.includeArchivedSymbols.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_bool(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolsListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbol: Vec<spotware::ProtoOALightSymbol<'a>>,
    pub archivedSymbol: Vec<spotware::ProtoOAArchivedSymbol<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolsListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.symbol.push(r.read_message::<spotware::ProtoOALightSymbol>(bytes)?),
                Ok(34) => msg.archivedSymbol.push(r.read_message::<spotware::ProtoOAArchivedSymbol>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbolsListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbol.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.archivedSymbol.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbol { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.archivedSymbol { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolByIdReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolByIdReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASymbolByIdReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolByIdRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbol: Vec<spotware::ProtoOASymbol<'a>>,
    pub archivedSymbol: Vec<spotware::ProtoOAArchivedSymbol<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolByIdRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.symbol.push(r.read_message::<spotware::ProtoOASymbol>(bytes)?),
                Ok(34) => msg.archivedSymbol.push(r.read_message::<spotware::ProtoOAArchivedSymbol>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbolByIdRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbol.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.archivedSymbol.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbol { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.archivedSymbol { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolsForConversionReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub firstAssetId: i64,
    pub lastAssetId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASymbolsForConversionReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.firstAssetId = r.read_int64(bytes)?,
                Ok(32) => msg.lastAssetId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASymbolsForConversionReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.firstAssetId) as u64)
        + 1 + sizeof_varint(*(&self.lastAssetId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.firstAssetId))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.lastAssetId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolsForConversionRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbol: Vec<spotware::ProtoOALightSymbol<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolsForConversionRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.symbol.push(r.read_message::<spotware::ProtoOALightSymbol>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbolsForConversionRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbol.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbol { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolChangedEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolChangedEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASymbolChangedEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAssetClassListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAssetClassListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAssetClassListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAssetClassListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub assetClass: Vec<spotware::ProtoOAAssetClass<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOAAssetClassListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.assetClass.push(r.read_message::<spotware::ProtoOAAssetClass>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAAssetClassListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.assetClass.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.assetClass { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATraderReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOATraderReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOATraderReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATraderRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub trader: spotware::ProtoOATrader<'a>,
}

impl<'a> MessageRead<'a> for ProtoOATraderRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.trader = r.read_message::<spotware::ProtoOATrader>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOATraderRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.trader).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.trader))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOATraderUpdatedEvent<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub trader: spotware::ProtoOATrader<'a>,
}

impl<'a> MessageRead<'a> for ProtoOATraderUpdatedEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.trader = r.read_message::<spotware::ProtoOATrader>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOATraderUpdatedEvent<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.trader).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.trader))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAReconcileReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub returnProtectionOrders: Option<bool>,
}

impl<'a> MessageRead<'a> for ProtoOAReconcileReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.returnProtectionOrders = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAReconcileReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.returnProtectionOrders.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.returnProtectionOrders.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_bool(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAReconcileRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub position: Vec<spotware::ProtoOAPosition<'a>>,
    pub order: Vec<spotware::ProtoOAOrder<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOAReconcileRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.position.push(r.read_message::<spotware::ProtoOAPosition>(bytes)?),
                Ok(34) => msg.order.push(r.read_message::<spotware::ProtoOAOrder>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAReconcileRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.position.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.order.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.position { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.order { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderErrorEvent<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub errorCode: Cow<'a, str>,
    pub orderId: Option<i64>,
    pub positionId: Option<i64>,
    pub description: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ProtoOAOrderErrorEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(40) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(18) => msg.errorCode = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.orderId = Some(r.read_int64(bytes)?),
                Ok(48) => msg.positionId = Some(r.read_int64(bytes)?),
                Ok(58) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAOrderErrorEvent<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((&self.errorCode).len())
        + self.orderId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.positionId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(18, |w| w.write_string(&self.errorCode))?;
        self.orderId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.positionId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(58, |w| w.write_string(&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub fromTimestamp: Option<i64>,
    pub toTimestamp: Option<i64>,
    pub maxRows: Option<i32>,
}

impl<'a> MessageRead<'a> for ProtoOADealListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(32) => msg.toTimestamp = Some(r.read_int64(bytes)?),
                Ok(40) => msg.maxRows = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.toTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.maxRows.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.toTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.maxRows.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealListRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub deal: Vec<spotware::ProtoOADeal>,
    pub hasMore: bool,
}

impl<'a> MessageRead<'a> for ProtoOADealListRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.deal.push(r.read_message::<spotware::ProtoOADeal>(bytes)?),
                Ok(32) => msg.hasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealListRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.deal.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.hasMore) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.deal { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_bool(*&self.hasMore))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub fromTimestamp: Option<i64>,
    pub toTimestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAOrderListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(32) => msg.toTimestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAOrderListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.toTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        self.toTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub order: Vec<spotware::ProtoOAOrder<'a>>,
    pub hasMore: bool,
}

impl<'a> MessageRead<'a> for ProtoOAOrderListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.order.push(r.read_message::<spotware::ProtoOAOrder>(bytes)?),
                Ok(32) => msg.hasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAOrderListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.order.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.hasMore) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.order { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_bool(*&self.hasMore))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAExpectedMarginReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: i64,
    pub volume: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAExpectedMarginReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId = r.read_int64(bytes)?,
                Ok(32) => msg.volume.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAExpectedMarginReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + self.volume.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.symbolId))?;
        for s in &self.volume { w.write_with_tag(32, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAExpectedMarginRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub margin: Vec<spotware::ProtoOAExpectedMargin>,
    pub moneyDigits: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOAExpectedMarginRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.margin.push(r.read_message::<spotware::ProtoOAExpectedMargin>(bytes)?),
                Ok(32) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAExpectedMarginRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.margin.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.margin { w.write_with_tag(26, |w| w.write_message(s))?; }
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginChangedEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: u64,
    pub usedMargin: u64,
    pub moneyDigits: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOAMarginChangedEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_uint64(bytes)?,
                Ok(32) => msg.usedMargin = r.read_uint64(bytes)?,
                Ok(40) => msg.moneyDigits = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginChangedEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_varint(*(&self.usedMargin) as u64)
        + self.moneyDigits.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_uint64(*&self.positionId))?;
        w.write_with_tag(32, |w| w.write_uint64(*&self.usedMargin))?;
        self.moneyDigits.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOACashFlowHistoryListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub fromTimestamp: i64,
    pub toTimestamp: i64,
}

impl<'a> MessageRead<'a> for ProtoOACashFlowHistoryListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.fromTimestamp = r.read_int64(bytes)?,
                Ok(32) => msg.toTimestamp = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOACashFlowHistoryListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.fromTimestamp) as u64)
        + 1 + sizeof_varint(*(&self.toTimestamp) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.fromTimestamp))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.toTimestamp))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOACashFlowHistoryListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub depositWithdraw: Vec<spotware::ProtoOADepositWithdraw<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOACashFlowHistoryListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.depositWithdraw.push(r.read_message::<spotware::ProtoOADepositWithdraw>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOACashFlowHistoryListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.depositWithdraw.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.depositWithdraw { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetAccountListByAccessTokenReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub accessToken: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOAGetAccountListByAccessTokenReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.accessToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAGetAccountListByAccessTokenReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.accessToken).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.accessToken))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetAccountListByAccessTokenRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub accessToken: Cow<'a, str>,
    pub permissionScope: Option<spotware::ProtoOAClientPermissionScope>,
    pub ctidTraderAccount: Vec<spotware::ProtoOACtidTraderAccount<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOAGetAccountListByAccessTokenRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.accessToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.permissionScope = Some(r.read_enum(bytes)?),
                Ok(34) => msg.ctidTraderAccount.push(r.read_message::<spotware::ProtoOACtidTraderAccount>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAGetAccountListByAccessTokenRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.accessToken).len())
        + self.permissionScope.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ctidTraderAccount.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.accessToken))?;
        self.permissionScope.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_enum(*&m as i32)))?;
        for s in &self.ctidTraderAccount { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeSpotsReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
    pub subscribeToSpotTimestamp: Option<bool>,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeSpotsReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(32) => msg.subscribeToSpotTimestamp = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeSpotsReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
        + self.subscribeToSpotTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        self.subscribeToSpotTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_bool(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeSpotsRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeSpotsRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeSpotsRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeSpotsReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeSpotsReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeSpotsReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeSpotsRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeSpotsRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeSpotsRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASpotEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: i64,
    pub bid: Option<u64>,
    pub ask: Option<u64>,
    pub trendbar: Vec<spotware::ProtoOATrendbar>,
    pub sessionClose: Option<u64>,
    pub timestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOASpotEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId = r.read_int64(bytes)?,
                Ok(32) => msg.bid = Some(r.read_uint64(bytes)?),
                Ok(40) => msg.ask = Some(r.read_uint64(bytes)?),
                Ok(50) => msg.trendbar.push(r.read_message::<spotware::ProtoOATrendbar>(bytes)?),
                Ok(56) => msg.sessionClose = Some(r.read_uint64(bytes)?),
                Ok(64) => msg.timestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASpotEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + self.bid.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.ask.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.trendbar.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.sessionClose.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.timestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.symbolId))?;
        self.bid.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_uint64(*&m)))?;
        self.ask.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_uint64(*&m)))?;
        for s in &self.trendbar { w.write_with_tag(50, |w| w.write_message(s))?; }
        self.sessionClose.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_uint64(*&m)))?;
        self.timestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeLiveTrendbarReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub period: spotware::ProtoOATrendbarPeriod,
    pub symbolId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeLiveTrendbarReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.period = r.read_enum(bytes)?,
                Ok(32) => msg.symbolId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeLiveTrendbarReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.period) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.period as i32))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.symbolId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeLiveTrendbarRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeLiveTrendbarRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeLiveTrendbarRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeLiveTrendbarReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub period: spotware::ProtoOATrendbarPeriod,
    pub symbolId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeLiveTrendbarReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.period = r.read_enum(bytes)?,
                Ok(32) => msg.symbolId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeLiveTrendbarReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.period) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.period as i32))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.symbolId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeLiveTrendbarRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeLiveTrendbarRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeLiveTrendbarRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetTrendbarsReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub fromTimestamp: Option<i64>,
    pub period: spotware::ProtoOATrendbarPeriod,
    pub symbolId: i64,
    pub count: Option<u32>,
}

impl<'a> MessageRead<'a> for ProtoOAGetTrendbarsReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(40) => msg.period = r.read_enum(bytes)?,
                Ok(48) => msg.symbolId = r.read_int64(bytes)?,
                Ok(56) => msg.count = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetTrendbarsReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.period) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + self.count.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(24, |w| w.write_int64(*&m)))?;
        w.write_with_tag(40, |w| w.write_enum(*&self.period as i32))?;
        w.write_with_tag(48, |w| w.write_int64(*&self.symbolId))?;
        self.count.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_uint32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetTrendbarsRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub period: spotware::ProtoOATrendbarPeriod,
    pub trendbar: Vec<spotware::ProtoOATrendbar>,
    pub symbolId: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAGetTrendbarsRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.period = r.read_enum(bytes)?,
                Ok(42) => msg.trendbar.push(r.read_message::<spotware::ProtoOATrendbar>(bytes)?),
                Ok(48) => msg.symbolId = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetTrendbarsRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.period) as u64)
        + self.trendbar.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.symbolId.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_enum(*&self.period as i32))?;
        for s in &self.trendbar { w.write_with_tag(42, |w| w.write_message(s))?; }
        self.symbolId.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetTickDataReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: i64,
    pub type_pb: spotware::ProtoOAQuoteType,
    pub fromTimestamp: Option<i64>,
    pub toTimestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAGetTickDataReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId = r.read_int64(bytes)?,
                Ok(32) => msg.type_pb = r.read_enum(bytes)?,
                Ok(40) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(48) => msg.toTimestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetTickDataReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.toTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.symbolId))?;
        w.write_with_tag(32, |w| w.write_enum(*&self.type_pb as i32))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        self.toTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(48, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetTickDataRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub tickData: Vec<spotware::ProtoOATickData>,
    pub hasMore: bool,
}

impl<'a> MessageRead<'a> for ProtoOAGetTickDataRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.tickData.push(r.read_message::<spotware::ProtoOATickData>(bytes)?),
                Ok(32) => msg.hasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetTickDataRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.tickData.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.hasMore) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.tickData { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_bool(*&self.hasMore))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetCtidProfileByTokenReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub accessToken: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOAGetCtidProfileByTokenReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.accessToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAGetCtidProfileByTokenReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.accessToken).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.accessToken))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetCtidProfileByTokenRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub profile: spotware::ProtoOACtidProfile,
}

impl<'a> MessageRead<'a> for ProtoOAGetCtidProfileByTokenRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.profile = r.read_message::<spotware::ProtoOACtidProfile>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetCtidProfileByTokenRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((self.profile).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_message(&self.profile))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADepthEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: u64,
    pub newQuotes: Vec<spotware::ProtoOADepthQuote>,
    pub deletedQuotes: Vec<u64>,
}

impl<'a> MessageRead<'a> for ProtoOADepthEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId = r.read_uint64(bytes)?,
                Ok(34) => msg.newQuotes.push(r.read_message::<spotware::ProtoOADepthQuote>(bytes)?),
                Ok(42) => msg.deletedQuotes = r.read_packed(bytes, |r, bytes| Ok(r.read_uint64(bytes)?))?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADepthEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.symbolId) as u64)
        + self.newQuotes.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.deletedQuotes.is_empty() { 0 } else { 1 + sizeof_len(self.deletedQuotes.iter().map(|&s| sizeof_varint(*(&s) as u64)).sum::<usize>()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_uint64(*&self.symbolId))?;
        for s in &self.newQuotes { w.write_with_tag(34, |w| w.write_message(s))?; }
        w.write_packed_with_tag(42, &self.deletedQuotes, |w, &m| w.write_uint64(*&m), &|&m| sizeof_varint(*(&m) as u64))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeDepthQuotesReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeDepthQuotesReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeDepthQuotesReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASubscribeDepthQuotesRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASubscribeDepthQuotesRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASubscribeDepthQuotesRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeDepthQuotesReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolId: Vec<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeDepthQuotesReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.symbolId.push(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeDepthQuotesReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolId.iter().map(|&s| 1 + sizeof_varint(*(&s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolId { w.write_with_tag(24, |w| w.write_int64(*&*s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAUnsubscribeDepthQuotesRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAUnsubscribeDepthQuotesRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAUnsubscribeDepthQuotesRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolCategoryListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOASymbolCategoryListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOASymbolCategoryListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOASymbolCategoryListRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub symbolCategory: Vec<spotware::ProtoOASymbolCategory<'a>>,
}

impl<'a> MessageRead<'a> for ProtoOASymbolCategoryListRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.symbolCategory.push(r.read_message::<spotware::ProtoOASymbolCategory>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOASymbolCategoryListRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.symbolCategory.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.symbolCategory { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountLogoutReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAccountLogoutReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAccountLogoutReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountLogoutRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAccountLogoutRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAccountLogoutRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAAccountDisconnectEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAAccountDisconnectEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAAccountDisconnectEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallListRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub marginCall: Vec<spotware::ProtoOAMarginCall>,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallListRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.marginCall.push(r.read_message::<spotware::ProtoOAMarginCall>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallListRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.marginCall.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        for s in &self.marginCall { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallUpdateReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub marginCall: spotware::ProtoOAMarginCall,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallUpdateReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.marginCall = r.read_message::<spotware::ProtoOAMarginCall>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallUpdateReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.marginCall).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.marginCall))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallUpdateRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallUpdateRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallUpdateRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallUpdateEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub marginCall: spotware::ProtoOAMarginCall,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallUpdateEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.marginCall = r.read_message::<spotware::ProtoOAMarginCall>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallUpdateEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.marginCall).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.marginCall))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAMarginCallTriggerEvent {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub marginCall: spotware::ProtoOAMarginCall,
}

impl<'a> MessageRead<'a> for ProtoOAMarginCallTriggerEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.marginCall = r.read_message::<spotware::ProtoOAMarginCall>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAMarginCallTriggerEvent {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.marginCall).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.marginCall))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetDynamicLeverageByIDReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub leverageId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAGetDynamicLeverageByIDReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.leverageId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetDynamicLeverageByIDReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.leverageId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.leverageId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetDynamicLeverageByIDRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub leverage: spotware::ProtoOADynamicLeverage,
}

impl<'a> MessageRead<'a> for ProtoOAGetDynamicLeverageByIDRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.leverage = r.read_message::<spotware::ProtoOADynamicLeverage>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetDynamicLeverageByIDRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.leverage).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.leverage))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOARefreshTokenReq<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub refreshToken: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOARefreshTokenReq<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.refreshToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOARefreshTokenReq<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.refreshToken).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.refreshToken))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOARefreshTokenRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub accessToken: Cow<'a, str>,
    pub tokenType: Cow<'a, str>,
    pub expiresIn: i64,
    pub refreshToken: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ProtoOARefreshTokenRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(18) => msg.accessToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.tokenType = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.expiresIn = r.read_int64(bytes)?,
                Ok(42) => msg.refreshToken = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOARefreshTokenRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_len((&self.accessToken).len())
        + 1 + sizeof_len((&self.tokenType).len())
        + 1 + sizeof_varint(*(&self.expiresIn) as u64)
        + 1 + sizeof_len((&self.refreshToken).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(18, |w| w.write_string(&self.accessToken))?;
        w.write_with_tag(26, |w| w.write_string(&self.tokenType))?;
        w.write_with_tag(32, |w| w.write_int64(*&self.expiresIn))?;
        w.write_with_tag(42, |w| w.write_string(&self.refreshToken))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAHoliday<'a> {
    pub holidayId: i64,
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub scheduleTimeZone: Cow<'a, str>,
    pub holidayDate: i64,
    pub isRecurring: bool,
    pub startSecond: Option<i32>,
    pub endSecond: Option<i32>,
}

impl<'a> MessageRead<'a> for ProtoOAHoliday<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.holidayId = r.read_int64(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.description = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(34) => msg.scheduleTimeZone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.holidayDate = r.read_int64(bytes)?,
                Ok(48) => msg.isRecurring = r.read_bool(bytes)?,
                Ok(56) => msg.startSecond = Some(r.read_int32(bytes)?),
                Ok(64) => msg.endSecond = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAHoliday<'a> {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.holidayId) as u64)
        + 1 + sizeof_len((&self.name).len())
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((&m).len()))
        + 1 + sizeof_len((&self.scheduleTimeZone).len())
        + 1 + sizeof_varint(*(&self.holidayDate) as u64)
        + 1 + sizeof_varint(*(&self.isRecurring) as u64)
        + self.startSecond.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.endSecond.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.holidayId))?;
        w.write_with_tag(18, |w| w.write_string(&self.name))?;
        self.description.as_ref().map_or(Ok(()), |m| w.write_with_tag(26, |w| w.write_string(&m)))?;
        w.write_with_tag(34, |w| w.write_string(&self.scheduleTimeZone))?;
        w.write_with_tag(40, |w| w.write_int64(*&self.holidayDate))?;
        w.write_with_tag(48, |w| w.write_bool(*&self.isRecurring))?;
        self.startSecond.as_ref().map_or(Ok(()), |&m| w.write_with_tag(56, |w| w.write_int32(*&m)))?;
        self.endSecond.as_ref().map_or(Ok(()), |&m| w.write_with_tag(64, |w| w.write_int32(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADynamicLeverage {
    pub leverageId: i64,
    pub tiers: Vec<spotware::ProtoOADynamicLeverageTier>,
}

impl<'a> MessageRead<'a> for ProtoOADynamicLeverage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.leverageId = r.read_int64(bytes)?,
                Ok(18) => msg.tiers.push(r.read_message::<spotware::ProtoOADynamicLeverageTier>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADynamicLeverage {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.leverageId) as u64)
        + self.tiers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.leverageId))?;
        for s in &self.tiers { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADynamicLeverageTier {
    pub volume: i64,
    pub leverage: i32,
}

impl<'a> MessageRead<'a> for ProtoOADynamicLeverageTier {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.volume = r.read_int64(bytes)?,
                Ok(16) => msg.leverage = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADynamicLeverageTier {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.volume) as u64)
        + 1 + sizeof_varint(*(&self.leverage) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.volume))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.leverage))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAPositionUnrealizedPnL {
    pub positionId: i64,
    pub grossUnrealizedPnL: i64,
    pub netUnrealizedPnL: i64,
}

impl<'a> MessageRead<'a> for ProtoOAPositionUnrealizedPnL {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.positionId = r.read_int64(bytes)?,
                Ok(16) => msg.grossUnrealizedPnL = r.read_int64(bytes)?,
                Ok(24) => msg.netUnrealizedPnL = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAPositionUnrealizedPnL {
    fn get_size(&self) -> usize {
        1 + sizeof_varint(*(&self.positionId) as u64)
        + 1 + sizeof_varint(*(&self.grossUnrealizedPnL) as u64)
        + 1 + sizeof_varint(*(&self.netUnrealizedPnL) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int64(*&self.positionId))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.grossUnrealizedPnL))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.netUnrealizedPnL))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealListByPositionIdReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: i64,
    pub fromTimestamp: Option<i64>,
    pub toTimestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOADealListByPositionIdReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(32) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(40) => msg.toTimestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealListByPositionIdReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.toTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.toTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealListByPositionIdRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub deal: Vec<spotware::ProtoOADeal>,
    pub hasMore: bool,
}

impl<'a> MessageRead<'a> for ProtoOADealListByPositionIdRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.deal.push(r.read_message::<spotware::ProtoOADeal>(bytes)?),
                Ok(32) => msg.hasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealListByPositionIdRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.deal.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.hasMore) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.deal { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_bool(*&self.hasMore))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderDetailsReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub orderId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAOrderDetailsReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.orderId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAOrderDetailsReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.orderId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.orderId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderDetailsRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub order: spotware::ProtoOAOrder<'a>,
    pub deal: Vec<spotware::ProtoOADeal>,
}

impl<'a> MessageRead<'a> for ProtoOAOrderDetailsRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.order = r.read_message::<spotware::ProtoOAOrder>(bytes)?,
                Ok(34) => msg.deal.push(r.read_message::<spotware::ProtoOADeal>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAOrderDetailsRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_len((self.order).get_size())
        + self.deal.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(26, |w| w.write_message(&self.order))?;
        for s in &self.deal { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderListByPositionIdReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionId: i64,
    pub fromTimestamp: Option<i64>,
    pub toTimestamp: Option<i64>,
}

impl<'a> MessageRead<'a> for ProtoOAOrderListByPositionIdReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.positionId = r.read_int64(bytes)?,
                Ok(32) => msg.fromTimestamp = Some(r.read_int64(bytes)?),
                Ok(40) => msg.toTimestamp = Some(r.read_int64(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAOrderListByPositionIdReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.positionId) as u64)
        + self.fromTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + self.toTimestamp.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.positionId))?;
        self.fromTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(32, |w| w.write_int64(*&m)))?;
        self.toTimestamp.as_ref().map_or(Ok(()), |&m| w.write_with_tag(40, |w| w.write_int64(*&m)))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAOrderListByPositionIdRes<'a> {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub order: Vec<spotware::ProtoOAOrder<'a>>,
    pub hasMore: bool,
}

impl<'a> MessageRead<'a> for ProtoOAOrderListByPositionIdRes<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.order.push(r.read_message::<spotware::ProtoOAOrder>(bytes)?),
                Ok(32) => msg.hasMore = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProtoOAOrderListByPositionIdRes<'a> {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.order.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.hasMore) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.order { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_bool(*&self.hasMore))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealOffsetListReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub dealId: i64,
}

impl<'a> MessageRead<'a> for ProtoOADealOffsetListReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(24) => msg.dealId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealOffsetListReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + 1 + sizeof_varint(*(&self.dealId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        w.write_with_tag(24, |w| w.write_int64(*&self.dealId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOADealOffsetListRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub offsetBy: Vec<spotware::ProtoOADealOffset>,
    pub offsetting: Vec<spotware::ProtoOADealOffset>,
}

impl<'a> MessageRead<'a> for ProtoOADealOffsetListRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.offsetBy.push(r.read_message::<spotware::ProtoOADealOffset>(bytes)?),
                Ok(34) => msg.offsetting.push(r.read_message::<spotware::ProtoOADealOffset>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOADealOffsetListRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.offsetBy.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.offsetting.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.offsetBy { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.offsetting { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetPositionUnrealizedPnLReq {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
}

impl<'a> MessageRead<'a> for ProtoOAGetPositionUnrealizedPnLReq {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetPositionUnrealizedPnLReq {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct ProtoOAGetPositionUnrealizedPnLRes {
    pub payloadType: Option<spotware::ProtoOAPayloadType>,
    pub ctidTraderAccountId: i64,
    pub positionUnrealizedPnL: Vec<spotware::ProtoOAPositionUnrealizedPnL>,
    pub moneyDigits: u32,
}

impl<'a> MessageRead<'a> for ProtoOAGetPositionUnrealizedPnLRes {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.payloadType = Some(r.read_enum(bytes)?),
                Ok(16) => msg.ctidTraderAccountId = r.read_int64(bytes)?,
                Ok(26) => msg.positionUnrealizedPnL.push(r.read_message::<spotware::ProtoOAPositionUnrealizedPnL>(bytes)?),
                Ok(32) => msg.moneyDigits = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ProtoOAGetPositionUnrealizedPnLRes {
    fn get_size(&self) -> usize {
        self.payloadType.as_ref().map_or(0, |&m| 1 + sizeof_varint(*(&m) as u64))
        + 1 + sizeof_varint(*(&self.ctidTraderAccountId) as u64)
        + self.positionUnrealizedPnL.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + 1 + sizeof_varint(*(&self.moneyDigits) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        self.payloadType.as_ref().map_or(Ok(()), |&m| w.write_with_tag(8, |w| w.write_enum(*&m as i32)))?;
        w.write_with_tag(16, |w| w.write_int64(*&self.ctidTraderAccountId))?;
        for s in &self.positionUnrealizedPnL { w.write_with_tag(26, |w| w.write_message(s))?; }
        w.write_with_tag(32, |w| w.write_uint32(*&self.moneyDigits))?;
        Ok(())
    }
}

