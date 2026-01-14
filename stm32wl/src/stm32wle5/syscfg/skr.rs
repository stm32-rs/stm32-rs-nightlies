///Register `SKR` writer
pub type W = crate::W<SKRrs>;
/**SRAM2 write protection key for software erase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY {
    ///17: Activate SRAM2ER bits write protection
    WriteProtect = 17,
    ///83: Step 2 to remove SRAM2ER bits write protection
    Step2 = 83,
    ///202: Step 1 to remove SRAM2ER bits write protection
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
impl crate::IsEnum for KEY {}
///Field `KEY` writer - SRAM2 write protection key for software erase
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Activate SRAM2ER bits write protection
    #[inline(always)]
    pub fn write_protect(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::WriteProtect)
    }
    ///Step 2 to remove SRAM2ER bits write protection
    #[inline(always)]
    pub fn step2(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Step2)
    }
    ///Step 1 to remove SRAM2ER bits write protection
    #[inline(always)]
    pub fn step1(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Step1)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SKRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - SRAM2 write protection key for software erase
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, SKRrs> {
        KEY_W::new(self, 0)
    }
}
/**SKR

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#SYSCFG:SKR)*/
pub struct SKRrs;
impl crate::RegisterSpec for SKRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`skr::W`](W) writer structure
impl crate::Writable for SKRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SKR to value 0
impl crate::Resettable for SKRrs {}
