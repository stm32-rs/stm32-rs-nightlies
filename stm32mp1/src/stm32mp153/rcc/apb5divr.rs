///Register `APB5DIVR` reader
pub type R = crate::R<APB5DIVRrs>;
///Register `APB5DIVR` writer
pub type W = crate::W<APB5DIVRrs>;
///Field `APB5DIV` reader - APB5DIV
pub type APB5DIV_R = crate::FieldReader;
///Field `APB5DIV` writer - APB5DIV
pub type APB5DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `APB5DIVRDY` reader - APB5DIVRDY
pub type APB5DIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - APB5DIV
    #[inline(always)]
    pub fn apb5div(&self) -> APB5DIV_R {
        APB5DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB5DIVRDY
    #[inline(always)]
    pub fn apb5divrdy(&self) -> APB5DIVRDY_R {
        APB5DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB5DIVR")
            .field("apb5div", &self.apb5div())
            .field("apb5divrdy", &self.apb5divrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - APB5DIV
    #[inline(always)]
    pub fn apb5div(&mut self) -> APB5DIV_W<'_, APB5DIVRrs> {
        APB5DIV_W::new(self, 0)
    }
}
/**This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.

You can [`read`](crate::Reg::read) this register and get [`apb5divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb5divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB5DIVR)*/
pub struct APB5DIVRrs;
impl crate::RegisterSpec for APB5DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`apb5divr::R`](R) reader structure
impl crate::Readable for APB5DIVRrs {}
///`write(|w| ..)` method takes [`apb5divr::W`](W) writer structure
impl crate::Writable for APB5DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB5DIVR to value 0x8000_0000
impl crate::Resettable for APB5DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
