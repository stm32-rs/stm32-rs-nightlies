///Register `SR2` reader
pub type R = crate::R<SR2rs>;
///Field `FLASH_RDY` reader - Flash ready flag
pub type FLASH_RDY_R = crate::BitReader;
///Field `REGLPS` reader - Low-power regulator started
pub type REGLPS_R = crate::BitReader;
///Field `REGLPF` reader - Low-power regulator flag
pub type REGLPF_R = crate::BitReader;
///Field `VOSF` reader - Voltage scaling flag
pub type VOSF_R = crate::BitReader;
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader;
///Field `PVMOUSB` reader - USB supply voltage monitoring output flag
pub type PVMOUSB_R = crate::BitReader;
///Field `PVMODAC` reader - VDDA monitoring output flag
pub type PVMODAC_R = crate::BitReader;
impl R {
    ///Bit 7 - Flash ready flag
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Low-power regulator started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - USB supply voltage monitoring output flag
    #[inline(always)]
    pub fn pvmousb(&self) -> PVMOUSB_R {
        PVMOUSB_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - VDDA monitoring output flag
    #[inline(always)]
    pub fn pvmodac(&self) -> PVMODAC_R {
        PVMODAC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR2")
            .field("pvmodac", &self.pvmodac())
            .field("pvmousb", &self.pvmousb())
            .field("pvdo", &self.pvdo())
            .field("vosf", &self.vosf())
            .field("reglpf", &self.reglpf())
            .field("reglps", &self.reglps())
            .field("flash_rdy", &self.flash_rdy())
            .finish()
    }
}
/**Power status register 2

You can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G051.html#PWR:SR2)*/
pub struct SR2rs;
impl crate::RegisterSpec for SR2rs {
    type Ux = u32;
}
///`read()` method returns [`sr2::R`](R) reader structure
impl crate::Readable for SR2rs {}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2rs {}
