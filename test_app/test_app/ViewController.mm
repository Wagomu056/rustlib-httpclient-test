//
//  ViewController.m
//  test_app
//
//  Created by 東口拓也 on 2023/03/11.
//

#import "ViewController.h"
#import "httpclient.h"

@interface ViewController ()

@end

@implementation ViewController

- (void)viewDidLoad {
    [super viewDidLoad];
    // Do any additional setup after loading the view.
    
    const char* rust_char = rust_hello();
    NSString* nsStr = [NSString stringWithCString:rust_char encoding:NSUTF8StringEncoding];
    NSLog(@"%@", nsStr);
    
    NSLog(@"prev request");
    http_request( [](bool is_success){
        NSLog(@"request success: %d", is_success ? 1 : 0);
    } );
    NSLog(@"post request");
}


@end
