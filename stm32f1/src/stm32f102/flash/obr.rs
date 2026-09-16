///Register `OBR` reader
pub type R = crate::R<OBRrs>;
///Field `OPTERR` reader - Option byte error
pub type OPTERR_R = crate::BitReader;
///Field `RDPRT` reader - Read protection
pub type RDPRT_R = crate::BitReader;
///Field `WDG_SW` reader - WDG_SW
pub type WDG_SW_R = crate::BitReader;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `Data0` reader - Data0
pub type DATA0_R = crate::FieldReader;
///Field `Data1` reader - Data1
pub type DATA1_R = crate::FieldReader;
impl R {
    ///Bit 0 - Option byte error
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read protection
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WDG_SW
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 10:17 - Data0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    ///Bits 18:25 - Data1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBR")
            .field("opterr", &self.opterr())
            .field("rdprt", &self.rdprt())
            .field("wdg_sw", &self.wdg_sw())
            .field("n_rst_stop", &self.n_rst_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
/**Option byte register

You can [`read`](crate::Reg::read) this register and get [`obr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F102.html#FLASH:OBR)*/
pub struct OBRrs;
impl crate::RegisterSpec for OBRrs {
    type Ux = u32;
}
///`read()` method returns [`obr::R`](R) reader structure
impl crate::Readable for OBRrs {}
///`reset()` method sets OBR to value 0x03ff_fffc
impl crate::Resettable for OBRrs {
    const RESET_VALUE: u32 = 0x03ff_fffc;
}
