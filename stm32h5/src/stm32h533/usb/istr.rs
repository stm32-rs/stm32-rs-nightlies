///Register `ISTR` reader
pub type R = crate::R<ISTRrs>;
///Register `ISTR` writer
pub type W = crate::W<ISTRrs>;
///Field `IDN` reader - Device Endpoint / host channel identification number
pub type IDN_R = crate::FieldReader;
///Field `DIR` reader - Direction of transaction
pub type DIR_R = crate::BitReader;
///Field `L1REQ` reader - LPM L1 state request
pub type L1REQ_R = crate::BitReader;
///Field `L1REQ` writer - LPM L1 state request
pub type L1REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ESOF` reader - Expected start of frame
pub type ESOF_R = crate::BitReader;
///Field `ESOF` writer - Expected start of frame
pub type ESOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOF` reader - Start of frame
pub type SOF_R = crate::BitReader;
///Field `SOF` writer - Start of frame
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_DCON` reader - USB reset request (Device mode) or device connect/disconnect (Host mode)
pub type RST_DCON_R = crate::BitReader;
///Field `RST_DCON` writer - USB reset request (Device mode) or device connect/disconnect (Host mode)
pub type RST_DCON_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSP` reader - Suspend mode request
pub type SUSP_R = crate::BitReader;
///Field `SUSP` writer - Suspend mode request
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUP` reader - Wake-up
pub type WKUP_R = crate::BitReader;
///Field `WKUP` writer - Wake-up
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR` reader - Error
pub type ERR_R = crate::BitReader;
///Field `ERR` writer - Error
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMAOVR` reader - Packet memory area over / underrun
pub type PMAOVR_R = crate::BitReader;
///Field `PMAOVR` writer - Packet memory area over / underrun
pub type PMAOVR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTR` reader - Completed transfer in host mode
pub type CTR_R = crate::BitReader;
///Field `THR512` reader - 512 byte threshold interrupt
pub type THR512_R = crate::BitReader;
///Field `THR512` writer - 512 byte threshold interrupt
pub type THR512_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DDISC` reader - Device connection
pub type DDISC_R = crate::BitReader;
///Field `DDISC` writer - Device connection
pub type DDISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCON_STAT` reader - Device connection status
pub type DCON_STAT_R = crate::BitReader;
///Field `LS_DCON` reader - Low speed device connected
pub type LS_DCON_R = crate::BitReader;
impl R {
    ///Bits 0:3 - Device Endpoint / host channel identification number
    #[inline(always)]
    pub fn idn(&self) -> IDN_R {
        IDN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Direction of transaction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start of frame
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start of frame
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode)
    #[inline(always)]
    pub fn rst_dcon(&self) -> RST_DCON_R {
        RST_DCON_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wake-up
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Completed transfer in host mode
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - 512 byte threshold interrupt
    #[inline(always)]
    pub fn thr512(&self) -> THR512_R {
        THR512_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Device connection
    #[inline(always)]
    pub fn ddisc(&self) -> DDISC_R {
        DDISC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 29 - Device connection status
    #[inline(always)]
    pub fn dcon_stat(&self) -> DCON_STAT_R {
        DCON_STAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Low speed device connected
    #[inline(always)]
    pub fn ls_dcon(&self) -> LS_DCON_R {
        LS_DCON_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTR")
            .field("idn", &self.idn())
            .field("dir", &self.dir())
            .field("l1req", &self.l1req())
            .field("esof", &self.esof())
            .field("sof", &self.sof())
            .field("rst_dcon", &self.rst_dcon())
            .field("susp", &self.susp())
            .field("wkup", &self.wkup())
            .field("err", &self.err())
            .field("pmaovr", &self.pmaovr())
            .field("ctr", &self.ctr())
            .field("thr512", &self.thr512())
            .field("ddisc", &self.ddisc())
            .field("dcon_stat", &self.dcon_stat())
            .field("ls_dcon", &self.ls_dcon())
            .finish()
    }
}
impl W {
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&mut self) -> L1REQ_W<ISTRrs> {
        L1REQ_W::new(self, 7)
    }
    ///Bit 8 - Expected start of frame
    #[inline(always)]
    pub fn esof(&mut self) -> ESOF_W<ISTRrs> {
        ESOF_W::new(self, 8)
    }
    ///Bit 9 - Start of frame
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<ISTRrs> {
        SOF_W::new(self, 9)
    }
    ///Bit 10 - USB reset request (Device mode) or device connect/disconnect (Host mode)
    #[inline(always)]
    pub fn rst_dcon(&mut self) -> RST_DCON_W<ISTRrs> {
        RST_DCON_W::new(self, 10)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W<ISTRrs> {
        SUSP_W::new(self, 11)
    }
    ///Bit 12 - Wake-up
    #[inline(always)]
    pub fn wkup(&mut self) -> WKUP_W<ISTRrs> {
        WKUP_W::new(self, 12)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W<ISTRrs> {
        ERR_W::new(self, 13)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&mut self) -> PMAOVR_W<ISTRrs> {
        PMAOVR_W::new(self, 14)
    }
    ///Bit 16 - 512 byte threshold interrupt
    #[inline(always)]
    pub fn thr512(&mut self) -> THR512_W<ISTRrs> {
        THR512_W::new(self, 16)
    }
    ///Bit 17 - Device connection
    #[inline(always)]
    pub fn ddisc(&mut self) -> DDISC_W<ISTRrs> {
        DDISC_W::new(self, 17)
    }
}
/**USB interrupt status register

You can [`read`](crate::Reg::read) this register and get [`istr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USB:ISTR)*/
pub struct ISTRrs;
impl crate::RegisterSpec for ISTRrs {
    type Ux = u32;
}
///`read()` method returns [`istr::R`](R) reader structure
impl crate::Readable for ISTRrs {}
///`write(|w| ..)` method takes [`istr::W`](W) writer structure
impl crate::Writable for ISTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ISTR to value 0
impl crate::Resettable for ISTRrs {
    const RESET_VALUE: u32 = 0;
}
