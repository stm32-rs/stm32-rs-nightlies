///Register `OBR` reader
pub type R = crate::R<OBRrs>;
///Field `RDPRT` reader - Read protection
pub type RDPRT_R = crate::FieldReader;
///Field `BOR_LEV` reader - BOR_LEV
pub type BOR_LEV_R = crate::FieldReader;
///Field `IWDG_SW` reader - IWDG_SW
pub type IWDG_SW_R = crate::BitReader;
///Field `nRTS_STOP` reader - nRTS_STOP
pub type N_RTS_STOP_R = crate::BitReader;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader;
///Field `BFB2` reader - Boot From Bank 2
pub type BFB2_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Read protection
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:19 - BOR_LEV
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - IWDG_SW
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - nRTS_STOP
    #[inline(always)]
    pub fn n_rts_stop(&self) -> N_RTS_STOP_R {
        N_RTS_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Boot From Bank 2
    #[inline(always)]
    pub fn bfb2(&self) -> BFB2_R {
        BFB2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OBR")
            .field("rdprt", &self.rdprt())
            .field("bor_lev", &self.bor_lev())
            .field("iwdg_sw", &self.iwdg_sw())
            .field("n_rts_stop", &self.n_rts_stop())
            .field("n_rst_stdby", &self.n_rst_stdby())
            .field("bfb2", &self.bfb2())
            .finish()
    }
}
/**Option byte register

You can [`read`](crate::Reg::read) this register and get [`obr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#Flash:OBR)*/
pub struct OBRrs;
impl crate::RegisterSpec for OBRrs {
    type Ux = u32;
}
///`read()` method returns [`obr::R`](R) reader structure
impl crate::Readable for OBRrs {}
///`reset()` method sets OBR to value 0x00f8_0000
impl crate::Resettable for OBRrs {
    const RESET_VALUE: u32 = 0x00f8_0000;
}
