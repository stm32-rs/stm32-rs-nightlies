///Register `IWDG_PR` reader
pub type R = crate::R<IWDG_PRrs>;
///Register `IWDG_PR` writer
pub type W = crate::W<IWDG_PRrs>;
///Field `PR` reader - Prescaler divider These bits are write access protected, see Section126.4.6. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
pub type PR_R = crate::FieldReader;
///Field `PR` writer - Prescaler divider These bits are write access protected, see Section126.4.6. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Prescaler divider These bits are write access protected, see Section126.4.6. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_PR").field("pr", &self.pr()).finish()
    }
}
impl W {
    ///Bits 0:3 - Prescaler divider These bits are write access protected, see Section126.4.6. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset to be able to change the prescaler divider. Others: divider / 1024 Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<IWDG_PRrs> {
        PR_W::new(self, 0)
    }
}
/**IWDG prescaler register

You can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#IWDG:IWDG_PR)*/
pub struct IWDG_PRrs;
impl crate::RegisterSpec for IWDG_PRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_pr::R`](R) reader structure
impl crate::Readable for IWDG_PRrs {}
///`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure
impl crate::Writable for IWDG_PRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IWDG_PR to value 0
impl crate::Resettable for IWDG_PRrs {
    const RESET_VALUE: u32 = 0;
}
