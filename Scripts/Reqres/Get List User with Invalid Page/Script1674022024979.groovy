import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

'it failed because reqres redirect me to first page instead of error page'
response1 = WS.sendRequest(findTestObject('Get List User with special char page', [('url') : GlobalVariable.baseUrl, ('page') : GlobalVariable.specialCharPage]))

WS.verifyResponseStatusCode(response1, 400)

'it failed because reqres redirect me to first page instead of error page'
response2 = WS.sendRequest(findTestObject('Get List User with string page', [('url') : GlobalVariable.baseUrl, ('page') : GlobalVariable.stringPage]))

WS.verifyResponseStatusCode(response2, 400)

'it failed because reqres redirect me to first page instead of error page'
response3 = WS.sendRequest(findTestObject('Get List User with double'))

WS.verifyResponseStatusCode(response3, 400)

'it failed because reqres redirect me to first page instead of error page'
response4 = WS.sendRequest(findTestObject('Get List User with exceeded page', [('url') : GlobalVariable.baseUrl, ('page') : GlobalVariable.exceedPage]))

WS.verifyResponseStatusCode(response4, 400)

