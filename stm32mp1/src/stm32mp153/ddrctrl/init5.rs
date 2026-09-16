///Register `INIT5` reader
pub type R = crate::R<INIT5rs>;
///Register `INIT5` writer
pub type W = crate::W<INIT5rs>;
///Field `MAX_AUTO_INIT_X1024` reader - MAX_AUTO_INIT_X1024
pub type MAX_AUTO_INIT_X1024_R = crate::FieldReader<u16>;
///Field `MAX_AUTO_INIT_X1024` writer - MAX_AUTO_INIT_X1024
pub type MAX_AUTO_INIT_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `DEV_ZQINIT_X32` reader - DEV_ZQINIT_X32
pub type DEV_ZQINIT_X32_R = crate::FieldReader;
///Field `DEV_ZQINIT_X32` writer - DEV_ZQINIT_X32
pub type DEV_ZQINIT_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:9 - MAX_AUTO_INIT_X1024
    #[inline(always)]
    pub fn max_auto_init_x1024(&self) -> MAX_AUTO_INIT_X1024_R {
        MAX_AUTO_INIT_X1024_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:23 - DEV_ZQINIT_X32
    #[inline(always)]
    pub fn dev_zqinit_x32(&self) -> DEV_ZQINIT_X32_R {
        DEV_ZQINIT_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INIT5")
            .field("max_auto_init_x1024", &self.max_auto_init_x1024())
            .field("dev_zqinit_x32", &self.dev_zqinit_x32())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - MAX_AUTO_INIT_X1024
    #[inline(always)]
    pub fn max_auto_init_x1024(&mut self) -> MAX_AUTO_INIT_X1024_W<'_, INIT5rs> {
        MAX_AUTO_INIT_X1024_W::new(self, 0)
    }
    ///Bits 16:23 - DEV_ZQINIT_X32
    #[inline(always)]
    pub fn dev_zqinit_x32(&mut self) -> DEV_ZQINIT_X32_W<'_, INIT5rs> {
        DEV_ZQINIT_X32_W::new(self, 16)
    }
}
/**DDRCTRL SDRAM initialization register 5

You can [`read`](crate::Reg::read) this register and get [`init5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:INIT5)*/
pub struct INIT5rs;
impl crate::RegisterSpec for INIT5rs {
    type Ux = u32;
}
///`read()` method returns [`init5::R`](R) reader structure
impl crate::Readable for INIT5rs {}
///`write(|w| ..)` method takes [`init5::W`](W) writer structure
impl crate::Writable for INIT5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets INIT5 to value 0x0010_0004
impl crate::Resettable for INIT5rs {
    const RESET_VALUE: u32 = 0x0010_0004;
}
