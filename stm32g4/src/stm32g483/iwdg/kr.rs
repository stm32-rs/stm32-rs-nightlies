#[doc = "Register `KR` writer"]
pub type W = crate::W<KRrs>;
#[doc = "Key value (write only, read 0x0000)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum KEY {
    #[doc = "21845: Enable access to PR, RLR and WINR registers (0x5555)"]
    Enable = 21845,
    #[doc = "43690: Reset the watchdog value (0xAAAA)"]
    Reset = 43690,
    #[doc = "52428: Start the watchdog (0xCCCC)"]
    Start = 52428,
}
impl From<KEY> for u16 {
    #[inline(always)]
    fn from(variant: KEY) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY {
    type Ux = u16;
}
#[doc = "Field `KEY` writer - Key value (write only, read 0x0000)"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Enable)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Reset)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Start)
    }
}
impl W {
    #[doc = "Bits 0:15 - Key value (write only, read 0x0000)"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<KRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "Key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KRrs;
impl crate::RegisterSpec for KRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`kr::W`](W) writer structure"]
impl crate::Writable for KRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KRrs {
    const RESET_VALUE: u32 = 0;
}
