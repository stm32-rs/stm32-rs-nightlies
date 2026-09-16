///Register `APB3DIVR` reader
pub type R = crate::R<APB3DIVRrs>;
///Register `APB3DIVR` writer
pub type W = crate::W<APB3DIVRrs>;
///Field `APB3DIV` reader - APB3DIV
pub type APB3DIV_R = crate::FieldReader;
///Field `APB3DIV` writer - APB3DIV
pub type APB3DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `APB3DIVRDY` reader - APB3DIVRDY
pub type APB3DIVRDY_R = crate::BitReader;
impl R {
    ///Bits 0:2 - APB3DIV
    #[inline(always)]
    pub fn apb3div(&self) -> APB3DIV_R {
        APB3DIV_R::new((self.bits & 7) as u8)
    }
    ///Bit 31 - APB3DIVRDY
    #[inline(always)]
    pub fn apb3divrdy(&self) -> APB3DIVRDY_R {
        APB3DIVRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB3DIVR")
            .field("apb3div", &self.apb3div())
            .field("apb3divrdy", &self.apb3divrdy())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - APB3DIV
    #[inline(always)]
    pub fn apb3div(&mut self) -> APB3DIV_W<'_, APB3DIVRrs> {
        APB3DIV_W::new(self, 0)
    }
}
/**This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`apb3divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:APB3DIVR)*/
pub struct APB3DIVRrs;
impl crate::RegisterSpec for APB3DIVRrs {
    type Ux = u32;
}
///`read()` method returns [`apb3divr::R`](R) reader structure
impl crate::Readable for APB3DIVRrs {}
///`write(|w| ..)` method takes [`apb3divr::W`](W) writer structure
impl crate::Writable for APB3DIVRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB3DIVR to value 0x8000_0000
impl crate::Resettable for APB3DIVRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
