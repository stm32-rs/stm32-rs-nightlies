#[doc = "Register `WPR` writer"]
pub type W = crate::W<WPRrs>;
#[doc = "Write protection key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY {
    #[doc = "0: Activate write protection (any value that is not the keys)"]
    Activate = 0,
    #[doc = "83: Key 2"]
    Deactivate2 = 83,
    #[doc = "202: Key 1"]
    Deactivate1 = 202,
}
impl From<KEY> for u8 {
    #[inline(always)]
    fn from(variant: KEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Write protection key"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Activate write protection (any value that is not the keys)"]
    #[inline(always)]
    pub fn activate(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Activate)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub fn deactivate2(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Deactivate2)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub fn deactivate1(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Deactivate1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write protection key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<WPRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "Write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPRrs;
impl crate::RegisterSpec for WPRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WPRrs {
    const RESET_VALUE: u32 = 0;
}
