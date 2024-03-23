#[doc = "Register `APB3FZ1` reader"]
pub type R = crate::R<APB3FZ1rs>;
#[doc = "Register `APB3FZ1` writer"]
pub type W = crate::W<APB3FZ1rs>;
#[doc = "Field `WWDG` reader - WWDG stop in debug"]
pub type WWDG_R = crate::BitReader;
#[doc = "Field `WWDG` writer - WWDG stop in debug"]
pub type WWDG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - WWDG stop in debug"]
    #[inline(always)]
    pub fn wwdg(&self) -> WWDG_R {
        WWDG_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WWDG stop in debug"]
    #[inline(always)]
    #[must_use]
    pub fn wwdg(&mut self) -> WWDG_W<APB3FZ1rs> {
        WWDG_W::new(self, 6)
    }
}
#[doc = "DBGMCU APB3 peripheral freeze register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3fz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3fz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3FZ1rs;
impl crate::RegisterSpec for APB3FZ1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3fz1::R`](R) reader structure"]
impl crate::Readable for APB3FZ1rs {}
#[doc = "`write(|w| ..)` method takes [`apb3fz1::W`](W) writer structure"]
impl crate::Writable for APB3FZ1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3FZ1 to value 0"]
impl crate::Resettable for APB3FZ1rs {
    const RESET_VALUE: u32 = 0;
}
