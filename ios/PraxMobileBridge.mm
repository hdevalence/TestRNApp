#import <React/RCTBridgeModule.h>
#import <React/RCTEventEmitter.h>

// Import the C header generated by cbindgen
#import "prax_mobile.h"

@interface PraxMobile : RCTEventEmitter <RCTBridgeModule>
@end

@implementation PraxMobile

RCT_EXPORT_MODULE();

RCT_EXPORT_METHOD(sum:(NSInteger)a withB:(NSInteger)b
                  resolver:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    NSInteger result = prax_mobile::sum((int32_t)a, (int32_t)b);
    resolve(@(result));
}

RCT_EXPORT_METHOD(startServer:(RCTPromiseResolveBlock)resolve
                  rejecter:(RCTPromiseRejectBlock)reject)
{
    dispatch_async(dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_DEFAULT, 0), ^{
        prax_mobile::start_server();
        dispatch_async(dispatch_get_main_queue(), ^{
            resolve(@"Server started");
        });
    });
}

@end