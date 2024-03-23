#[doc = "Register `DDRCTRL_INIT5` reader"]
pub type R = crate::R<DDRCTRL_INIT5rs>;
#[doc = "Register `DDRCTRL_INIT5` writer"]
pub type W = crate::W<DDRCTRL_INIT5rs>;
#[doc = "Field `MAX_AUTO_INIT_X1024` reader - MAX_AUTO_INIT_X1024"]
pub type MAX_AUTO_INIT_X1024_R = crate::FieldReader<u16>;
#[doc = "Field `MAX_AUTO_INIT_X1024` writer - MAX_AUTO_INIT_X1024"]
pub type MAX_AUTO_INIT_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DEV_ZQINIT_X32` reader - DEV_ZQINIT_X32"]
pub type DEV_ZQINIT_X32_R = crate::FieldReader;
#[doc = "Field `DEV_ZQINIT_X32` writer - DEV_ZQINIT_X32"]
pub type DEV_ZQINIT_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    pub fn max_auto_init_x1024(&self) -> MAX_AUTO_INIT_X1024_R {
        MAX_AUTO_INIT_X1024_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    pub fn dev_zqinit_x32(&self) -> DEV_ZQINIT_X32_R {
        DEV_ZQINIT_X32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - MAX_AUTO_INIT_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn max_auto_init_x1024(&mut self) -> MAX_AUTO_INIT_X1024_W<DDRCTRL_INIT5rs> {
        MAX_AUTO_INIT_X1024_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - DEV_ZQINIT_X32"]
    #[inline(always)]
    #[must_use]
    pub fn dev_zqinit_x32(&mut self) -> DEV_ZQINIT_X32_W<DDRCTRL_INIT5rs> {
        DEV_ZQINIT_X32_W::new(self, 16)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT5rs;
impl crate::RegisterSpec for DDRCTRL_INIT5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init5::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT5rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init5::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT5 to value 0x0010_0004"]
impl crate::Resettable for DDRCTRL_INIT5rs {
    const RESET_VALUE: u32 = 0x0010_0004;
}
