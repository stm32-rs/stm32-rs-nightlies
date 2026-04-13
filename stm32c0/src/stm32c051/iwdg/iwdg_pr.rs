///Register `IWDG_PR` reader
pub type R = crate::R<IWDG_PRrs>;
///Register `IWDG_PR` writer
pub type W = crate::W<IWDG_PRrs>;
/**Prescaler divider These bits are write access protected see Section 22.3.4: Register access protection. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR {
    ///0: divider /4
    B0x0 = 0,
    ///1: divider /8
    B0x1 = 1,
    ///2: divider /16
    B0x2 = 2,
    ///3: divider /32
    B0x3 = 3,
    ///4: divider /64
    B0x4 = 4,
    ///5: divider /128
    B0x5 = 5,
    ///6: divider /256
    B0x6 = 6,
    ///7: divider /256
    B0x7 = 7,
}
impl From<PR> for u8 {
    #[inline(always)]
    fn from(variant: PR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR {
    type Ux = u8;
}
impl crate::IsEnum for PR {}
///Field `PR` reader - Prescaler divider These bits are write access protected see Section 22.3.4: Register access protection. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
pub type PR_R = crate::FieldReader<PR>;
impl PR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PR {
        match self.bits {
            0 => PR::B0x0,
            1 => PR::B0x1,
            2 => PR::B0x2,
            3 => PR::B0x3,
            4 => PR::B0x4,
            5 => PR::B0x5,
            6 => PR::B0x6,
            7 => PR::B0x7,
            _ => unreachable!(),
        }
    }
    ///divider /4
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PR::B0x0
    }
    ///divider /8
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PR::B0x1
    }
    ///divider /16
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PR::B0x2
    }
    ///divider /32
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PR::B0x3
    }
    ///divider /64
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PR::B0x4
    }
    ///divider /128
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PR::B0x5
    }
    ///divider /256
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PR::B0x6
    }
    ///divider /256
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PR::B0x7
    }
}
///Field `PR` writer - Prescaler divider These bits are write access protected see Section 22.3.4: Register access protection. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PR, crate::Safe>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///divider /4
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x0)
    }
    ///divider /8
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x1)
    }
    ///divider /16
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x2)
    }
    ///divider /32
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x3)
    }
    ///divider /64
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x4)
    }
    ///divider /128
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x5)
    }
    ///divider /256
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x6)
    }
    ///divider /256
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(PR::B0x7)
    }
}
impl R {
    ///Bits 0:2 - Prescaler divider These bits are write access protected see Section 22.3.4: Register access protection. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IWDG_PR").field("pr", &self.pr()).finish()
    }
}
impl W {
    ///Bits 0:2 - Prescaler divider These bits are write access protected see Section 22.3.4: Register access protection. They are written by software to select the prescaler divider feeding the counter clock. PVU bit of the IWDG status register (IWDG_SR) must be reset in order to be able to change the prescaler divider. Note: Reading this register returns the prescaler value from the V<sub>DD</sub> voltage domain. This value may not be up to date/valid if a write operation to this register is ongoing. For this reason the value read from this register is valid only when the PVU bit in the IWDG status register (IWDG_SR) is reset.
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, IWDG_PRrs> {
        PR_W::new(self, 0)
    }
}
/**IWDG prescaler register

You can [`read`](crate::Reg::read) this register and get [`iwdg_pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdg_pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#IWDG:IWDG_PR)*/
pub struct IWDG_PRrs;
impl crate::RegisterSpec for IWDG_PRrs {
    type Ux = u32;
}
///`read()` method returns [`iwdg_pr::R`](R) reader structure
impl crate::Readable for IWDG_PRrs {}
///`write(|w| ..)` method takes [`iwdg_pr::W`](W) writer structure
impl crate::Writable for IWDG_PRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IWDG_PR to value 0
impl crate::Resettable for IWDG_PRrs {}
