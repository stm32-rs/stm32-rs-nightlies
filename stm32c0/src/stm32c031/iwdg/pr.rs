#[doc = "Register `PR` reader"]
pub type R = crate::R<PRrs>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PRrs>;
#[doc = "Field `PR` reader - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider These bits are write access protected see . They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the VDD voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the status register (IWDG_SR) is reset."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<PRrs> {
        PR_W::new(self, 0)
    }
}
#[doc = "IWDG prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRrs;
impl crate::RegisterSpec for PRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PRrs {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PRrs {
    const RESET_VALUE: u32 = 0;
}
