///Register `SRDCR` reader
pub type R = crate::R<SRDCRrs>;
///Register `SRDCR` writer
pub type W = crate::W<SRDCRrs>;
///Field `VOSRDY` reader - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3).
pub type VOSRDY_R = crate::BitReader;
///Field `VOS` reader - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling.
pub type VOS_R = crate::FieldReader;
///Field `VOS` writer - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling.
pub type VOS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 13 - VOS Ready bit for VCORE voltage scaling output selection. This bit is set to 1 by hardware when Bypass mode is selected in PWR control register 3 (PWR_CR3).
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRDCR")
            .field("vosrdy", &self.vosrdy())
            .field("vos", &self.vos())
            .finish()
    }
}
impl W {
    ///Bits 14:15 - Voltage scaling selection according to performance These bits control the VCORE voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling shall be changed before increasing the system frequency. When decreasing performance, the system frequency shall first be decreased before changing the voltage scaling.
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, SRDCRrs> {
        VOS_W::new(self, 14)
    }
}
/**This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software

You can [`read`](crate::Reg::read) this register and get [`srdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#PWR:SRDCR)*/
pub struct SRDCRrs;
impl crate::RegisterSpec for SRDCRrs {
    type Ux = u32;
}
///`read()` method returns [`srdcr::R`](R) reader structure
impl crate::Readable for SRDCRrs {}
///`write(|w| ..)` method takes [`srdcr::W`](W) writer structure
impl crate::Writable for SRDCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SRDCR to value 0x4000
impl crate::Resettable for SRDCRrs {
    const RESET_VALUE: u32 = 0x4000;
}
