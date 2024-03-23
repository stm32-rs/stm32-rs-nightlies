#[doc = "Register `SKR` writer"]
pub type W = crate::W<SKRrs>;
#[doc = "SRAM2 write protection key for software erase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY {
    #[doc = "17: Activate SRAM2ER bits write protection"]
    WriteProtect = 17,
    #[doc = "83: Step 2 to remove SRAM2ER bits write protection"]
    Step2 = 83,
    #[doc = "202: Step 1 to remove SRAM2ER bits write protection"]
    Step1 = 202,
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
#[doc = "Field `KEY` writer - SRAM2 write protection key for software erase"]
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Activate SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn write_protect(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::WriteProtect)
    }
    #[doc = "Step 2 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step2(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Step2)
    }
    #[doc = "Step 1 to remove SRAM2ER bits write protection"]
    #[inline(always)]
    pub fn step1(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Step1)
    }
}
impl W {
    #[doc = "Bits 0:7 - SRAM2 write protection key for software erase"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<SKRrs> {
        KEY_W::new(self, 0)
    }
}
#[doc = "SKR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SKRrs;
impl crate::RegisterSpec for SKRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`skr::W`](W) writer structure"]
impl crate::Writable for SKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SKR to value 0"]
impl crate::Resettable for SKRrs {
    const RESET_VALUE: u32 = 0;
}
