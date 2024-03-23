#[doc = "Register `WUSCR` writer"]
pub type W = crate::W<WUSCRrs>;
#[doc = "clear wakeup pin flag for WUFx These bits are always read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1W {
    #[doc = "1: Writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)"]
    Clear = 1,
}
impl From<CWUF1W> for bool {
    #[inline(always)]
    fn from(variant: CWUF1W) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWUF1` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub type CWUF1_W<'a, REG> = crate::BitWriter<'a, REG, CWUF1W>;
impl<'a, REG> CWUF1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing 1 clears the WUFx wakeup pin flag (bit is cleared to 0 by hardware)"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CWUF1W::Clear)
    }
}
#[doc = "Field `CWUF2` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF2_W;
#[doc = "Field `CWUF3` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF3_W;
#[doc = "Field `CWUF4` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF4_W;
#[doc = "Field `CWUF5` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF5_W;
#[doc = "Field `CWUF6` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF6_W;
#[doc = "Field `CWUF7` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF7_W;
#[doc = "Field `CWUF8` writer - clear wakeup pin flag for WUFx These bits are always read as 0."]
pub use CWUF1_W as CWUF8_W;
impl W {
    #[doc = "Bit 0 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<WUSCRrs> {
        CWUF1_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<WUSCRrs> {
        CWUF2_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<WUSCRrs> {
        CWUF3_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<WUSCRrs> {
        CWUF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<WUSCRrs> {
        CWUF5_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf6(&mut self) -> CWUF6_W<WUSCRrs> {
        CWUF6_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf7(&mut self) -> CWUF7_W<WUSCRrs> {
        CWUF7_W::new(self, 6)
    }
    #[doc = "Bit 7 - clear wakeup pin flag for WUFx These bits are always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn cwuf8(&mut self) -> CWUF8_W<WUSCRrs> {
        CWUF8_W::new(self, 7)
    }
}
#[doc = "PWR wakeup status clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuscr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WUSCRrs;
impl crate::RegisterSpec for WUSCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wuscr::W`](W) writer structure"]
impl crate::Writable for WUSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUSCR to value 0"]
impl crate::Resettable for WUSCRrs {
    const RESET_VALUE: u32 = 0;
}
