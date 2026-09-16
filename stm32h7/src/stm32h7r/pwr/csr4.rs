///Register `CSR4` reader
pub type R = crate::R<CSR4rs>;
///Register `CSR4` writer
pub type W = crate::W<CSR4rs>;
///Field `VOS` reader - Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
pub type VOS_R = crate::BitReader;
///Field `VOS` writer - Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
pub type VOS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VOSRDY` reader - VOS Ready bit
pub type VOSRDY_R = crate::BitReader;
impl R {
    ///Bit 0 - Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VOS Ready bit
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR4")
            .field("vos", &self.vos())
            .field("vosrdy", &self.vosrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - Voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtains the best trade-off between power consumption and performance: When increasing the performance, the voltage scaling must be changed before increasing the system frequency. When decreasing performance, the system frequency must first be decreased before changing the voltage scaling. Note: Refer to Section Electrical characteristics of the product datasheet for more details.
    #[inline(always)]
    pub fn vos(&mut self) -> VOS_W<'_, CSR4rs> {
        VOS_W::new(self, 0)
    }
}
/**PWR control status register 4

You can [`read`](crate::Reg::read) this register and get [`csr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#PWR:CSR4)*/
pub struct CSR4rs;
impl crate::RegisterSpec for CSR4rs {
    type Ux = u32;
}
///`read()` method returns [`csr4::R`](R) reader structure
impl crate::Readable for CSR4rs {}
///`write(|w| ..)` method takes [`csr4::W`](W) writer structure
impl crate::Writable for CSR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSR4 to value 0x02
impl crate::Resettable for CSR4rs {
    const RESET_VALUE: u32 = 0x02;
}
