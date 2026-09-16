///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**Master Compare %s Interrupt flag clear

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1CW {
    ///1: Clears associated flag in ISR register
    Clear = 1,
}
impl From<CMP1CW> for bool {
    #[inline(always)]
    fn from(variant: CMP1CW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPC(1-4)` writer - Master Compare %s Interrupt flag clear
pub type CMPC_W<'a, REG> = crate::BitWriter1C<'a, REG, CMP1CW>;
impl<'a, REG> CMPC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clears associated flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMP1CW::Clear)
    }
}
///Field `REPC` writer - Repetition Interrupt flag clear
pub use CMPC_W as REPC_W;
///Field `SYNCC` writer - Sync Input Interrupt flag clear
pub use CMPC_W as SYNCC_W;
///Field `UPDC` writer - Master update Interrupt flag clear
pub use CMPC_W as UPDC_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Master Compare (1-4) Interrupt flag clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1C` field.</div>
    #[inline(always)]
    pub fn cmpc(&mut self, n: u8) -> CMPC_W<'_, ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPC_W::new(self, n)
    }
    ///Bit 0 - Master Compare 1 Interrupt flag clear
    #[inline(always)]
    pub fn cmp1c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 0)
    }
    ///Bit 1 - Master Compare 2 Interrupt flag clear
    #[inline(always)]
    pub fn cmp2c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 1)
    }
    ///Bit 2 - Master Compare 3 Interrupt flag clear
    #[inline(always)]
    pub fn cmp3c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 2)
    }
    ///Bit 3 - Master Compare 4 Interrupt flag clear
    #[inline(always)]
    pub fn cmp4c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 3)
    }
    ///Bit 4 - Repetition Interrupt flag clear
    #[inline(always)]
    pub fn repc(&mut self) -> REPC_W<'_, ICRrs> {
        REPC_W::new(self, 4)
    }
    ///Bit 5 - Sync Input Interrupt flag clear
    #[inline(always)]
    pub fn syncc(&mut self) -> SYNCC_W<'_, ICRrs> {
        SYNCC_W::new(self, 5)
    }
    ///Bit 6 - Master update Interrupt flag clear
    #[inline(always)]
    pub fn updc(&mut self) -> UPDC_W<'_, ICRrs> {
        UPDC_W::new(self, 6)
    }
}
/**Master Timer Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#HRTIM_Master:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7f;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
