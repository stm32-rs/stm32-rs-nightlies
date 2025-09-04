///Register `CSR` reader
pub type R = crate::R<CSRrs>;
///Register `CSR` writer
pub type W = crate::W<CSRrs>;
///Field `ENVR` reader - Voltage reference buffer enable
pub type ENVR_R = crate::BitReader;
///Field `ENVR` writer - Voltage reference buffer enable
pub type ENVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HIZ` reader - High impedance mode
pub type HIZ_R = crate::BitReader;
///Field `HIZ` writer - High impedance mode
pub type HIZ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VRS` reader - Voltage reference scale
pub type VRS_R = crate::BitReader;
///Field `VRS` writer - Voltage reference scale
pub type VRS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VRR` reader - Voltage reference buffer ready
pub type VRR_R = crate::BitReader;
impl R {
    ///Bit 0 - Voltage reference buffer enable
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Voltage reference buffer ready
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("envr", &self.envr())
            .field("hiz", &self.hiz())
            .field("vrs", &self.vrs())
            .field("vrr", &self.vrr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage reference buffer enable
    #[inline(always)]
    pub fn envr(&mut self) -> ENVR_W<CSRrs> {
        ENVR_W::new(self, 0)
    }
    ///Bit 1 - High impedance mode
    #[inline(always)]
    pub fn hiz(&mut self) -> HIZ_W<CSRrs> {
        HIZ_W::new(self, 1)
    }
    ///Bit 2 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&mut self) -> VRS_W<CSRrs> {
        VRS_W::new(self, 2)
    }
}
/**VREF control and status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#VREFBUF:CSR)*/
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
///`read()` method returns [`csr::R`](R) reader structure
impl crate::Readable for CSRrs {}
///`write(|w| ..)` method takes [`csr::W`](W) writer structure
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR to value 0x02
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x02;
}
