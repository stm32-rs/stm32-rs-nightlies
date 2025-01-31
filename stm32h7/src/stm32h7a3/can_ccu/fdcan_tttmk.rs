///Register `FDCAN_TTTMK` reader
pub type R = crate::R<FDCAN_TTTMKrs>;
///Register `FDCAN_TTTMK` writer
pub type W = crate::W<FDCAN_TTTMKrs>;
///Field `TM` reader - Time Mark
pub type TM_R = crate::FieldReader<u16>;
///Field `TM` writer - Time Mark
pub type TM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TICC` reader - Time Mark Cycle Code
pub type TICC_R = crate::FieldReader;
///Field `TICC` writer - Time Mark Cycle Code
pub type TICC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `LCKM` reader - TT Time Mark Register Locked
pub type LCKM_R = crate::BitReader;
///Field `LCKM` writer - TT Time Mark Register Locked
pub type LCKM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Time Mark
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:22 - Time Mark Cycle Code
    #[inline(always)]
    pub fn ticc(&self) -> TICC_R {
        TICC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - TT Time Mark Register Locked
    #[inline(always)]
    pub fn lckm(&self) -> LCKM_R {
        LCKM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TTTMK")
            .field("tm", &self.tm())
            .field("ticc", &self.ticc())
            .field("lckm", &self.lckm())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Time Mark
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W<FDCAN_TTTMKrs> {
        TM_W::new(self, 0)
    }
    ///Bits 16:22 - Time Mark Cycle Code
    #[inline(always)]
    pub fn ticc(&mut self) -> TICC_W<FDCAN_TTTMKrs> {
        TICC_W::new(self, 16)
    }
    ///Bit 31 - TT Time Mark Register Locked
    #[inline(always)]
    pub fn lckm(&mut self) -> LCKM_W<FDCAN_TTTMKrs> {
        LCKM_W::new(self, 31)
    }
}
/**FDCAN TT Time Mark Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tttmk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#CAN_CCU:FDCAN_TTTMK)*/
pub struct FDCAN_TTTMKrs;
impl crate::RegisterSpec for FDCAN_TTTMKrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tttmk::R`](R) reader structure
impl crate::Readable for FDCAN_TTTMKrs {}
///`write(|w| ..)` method takes [`fdcan_tttmk::W`](W) writer structure
impl crate::Writable for FDCAN_TTTMKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TTTMK to value 0
impl crate::Resettable for FDCAN_TTTMKrs {
    const RESET_VALUE: u32 = 0;
}
