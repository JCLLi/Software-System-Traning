/*
 * generated by Xtext 2.29.0
 */
package xray


/**
 * Initialization support for running Xtext languages without Equinox extension registry.
 */
class XRayDSLStandaloneSetup extends XRayDSLStandaloneSetupGenerated {

	def static void doSetup() {
		new XRayDSLStandaloneSetup().createInjectorAndDoEMFRegistration()
	}
}
