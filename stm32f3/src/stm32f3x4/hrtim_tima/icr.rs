///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Compare %s Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMP1CW;
///Field `CMPC(1-4)` writer - Compare %s Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W;
///Field `REPC` writer - Repetition Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as REPC_W;
///Field `UPDC` writer - Update Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as UPDC_W;
///Field `CPTC(1-2)` writer - Capture%s Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as CPTC_W;
///Field `SETC(1-2)` writer - Output %s Set flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as SETC_W;
///Field `RST1C` writer - Output 1 Reset flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as RST1C_W;
///Field `RST2C` writer - Output 2 Reset flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as RST2C_W;
///Field `RSTC` writer - Reset Interrupt flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as RSTC_W;
///Field `DLYPRTC` writer - Delayed Protection Flag Clear
pub use crate::stm32f3x4::hrtim_master::icr::CMPC_W as DLYPRTC_W;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Compare (1-4) Interrupt flag Clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1C` field.</div>
    #[inline(always)]
    pub fn cmpc(&mut self, n: u8) -> CMPC_W<'_, ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMPC_W::new(self, n)
    }
    ///Bit 0 - Compare 1 Interrupt flag Clear
    #[inline(always)]
    pub fn cmp1c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 0)
    }
    ///Bit 1 - Compare 2 Interrupt flag Clear
    #[inline(always)]
    pub fn cmp2c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 1)
    }
    ///Bit 2 - Compare 3 Interrupt flag Clear
    #[inline(always)]
    pub fn cmp3c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 2)
    }
    ///Bit 3 - Compare 4 Interrupt flag Clear
    #[inline(always)]
    pub fn cmp4c(&mut self) -> CMPC_W<'_, ICRrs> {
        CMPC_W::new(self, 3)
    }
    ///Bit 4 - Repetition Interrupt flag Clear
    #[inline(always)]
    pub fn repc(&mut self) -> REPC_W<'_, ICRrs> {
        REPC_W::new(self, 4)
    }
    ///Bit 6 - Update Interrupt flag Clear
    #[inline(always)]
    pub fn updc(&mut self) -> UPDC_W<'_, ICRrs> {
        UPDC_W::new(self, 6)
    }
    ///Capture(1-2) Interrupt flag Clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1C` field.</div>
    #[inline(always)]
    pub fn cptc(&mut self, n: u8) -> CPTC_W<'_, ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPTC_W::new(self, n + 7)
    }
    ///Bit 7 - Capture1 Interrupt flag Clear
    #[inline(always)]
    pub fn cpt1c(&mut self) -> CPTC_W<'_, ICRrs> {
        CPTC_W::new(self, 7)
    }
    ///Bit 8 - Capture2 Interrupt flag Clear
    #[inline(always)]
    pub fn cpt2c(&mut self) -> CPTC_W<'_, ICRrs> {
        CPTC_W::new(self, 8)
    }
    ///Output (1-2) Set flag Clear
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1C` field.</div>
    #[inline(always)]
    pub fn setc(&mut self, n: u8) -> SETC_W<'_, ICRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SETC_W::new(self, n * 2 + 9)
    }
    ///Bit 9 - Output 1 Set flag Clear
    #[inline(always)]
    pub fn set1c(&mut self) -> SETC_W<'_, ICRrs> {
        SETC_W::new(self, 9)
    }
    ///Bit 11 - Output 2 Set flag Clear
    #[inline(always)]
    pub fn set2c(&mut self) -> SETC_W<'_, ICRrs> {
        SETC_W::new(self, 11)
    }
    ///Bit 10 - Output 1 Reset flag Clear
    #[inline(always)]
    pub fn rst1c(&mut self) -> RST1C_W<'_, ICRrs> {
        RST1C_W::new(self, 10)
    }
    ///Bit 12 - Output 2 Reset flag Clear
    #[inline(always)]
    pub fn rst2c(&mut self) -> RST2C_W<'_, ICRrs> {
        RST2C_W::new(self, 12)
    }
    ///Bit 13 - Reset Interrupt flag Clear
    #[inline(always)]
    pub fn rstc(&mut self) -> RSTC_W<'_, ICRrs> {
        RSTC_W::new(self, 13)
    }
    ///Bit 14 - Delayed Protection Flag Clear
    #[inline(always)]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<'_, ICRrs> {
        DLYPRTC_W::new(self, 14)
    }
}
/**Timerx Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIMA:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x7fdf;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
