///Register `KR` writer
pub type W = crate::W<KRrs>;
/**Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section 29.4.6: Register access protection) Writing the key value 0xCCCC starts the watchdog (except if the hardware watchdog option is selected)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum KEY {
    ///21845: Enable access to PR, RLR and WINR registers
    Unlock = 21845,
    ///43690: Feed watchdog with RLR register value
    Feed = 43690,
    ///52428: Start the watchdog
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
impl crate::IsEnum for KEY {}
///Field `KEY` writer - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section 29.4.6: Register access protection) Writing the key value 0xCCCC starts the watchdog (except if the hardware watchdog option is selected)
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, KEY>;
impl<'a, REG> KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///Enable access to PR, RLR and WINR registers
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Unlock)
    }
    ///Feed watchdog with RLR register value
    #[inline(always)]
    pub fn feed(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Feed)
    }
    ///Start the watchdog
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(KEY::Start)
    }
}
impl core::fmt::Debug for crate::generic::Reg<KRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Key value (write only, read 0x0000) These bits must be written by software at regular intervals with the key value 0xAAAA, otherwise the watchdog generates a reset when the counter reaches 0. Writing the key value 0x5555 to enable access to the IWDG_PR, IWDG_RLR and IWDG_WINR registers (see Section 29.4.6: Register access protection) Writing the key value 0xCCCC starts the watchdog (except if the hardware watchdog option is selected)
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<'_, KRrs> {
        KEY_W::new(self, 0)
    }
}
/**IWDG key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#IWDG:KR)*/
pub struct KRrs;
impl crate::RegisterSpec for KRrs {
    type Ux = u16;
}
///`write(|w| ..)` method takes [`kr::W`](W) writer structure
impl crate::Writable for KRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets KR to value 0
impl crate::Resettable for KRrs {}
