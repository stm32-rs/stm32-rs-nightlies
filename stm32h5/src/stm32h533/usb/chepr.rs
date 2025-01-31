///Register `CHEP%sR` reader
pub type R = crate::R<CHEPRrs>;
///Register `CHEP%sR` writer
pub type W = crate::W<CHEPRrs>;
///Field `EA` reader - endpoint/channel address
pub type EA_R = crate::FieldReader;
///Field `EA` writer - endpoint/channel address
pub type EA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `STATTX` writer - Status bits, for transmission transfers
pub type STATTX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOGTX` writer - Data toggle, for transmission transfers
pub type DTOGTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTTX` reader - Valid USB transaction transmitted
pub type VTTX_R = crate::BitReader;
///Field `VTTX` writer - Valid USB transaction transmitted
pub type VTTX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPKIND` reader - endpoint/channel kind
pub type EPKIND_R = crate::BitReader;
///Field `EPKIND` writer - endpoint/channel kind
pub type EPKIND_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UTYPE` reader - USB type of transaction
pub type UTYPE_R = crate::FieldReader;
///Field `UTYPE` writer - USB type of transaction
pub type UTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SETUP` reader - Setup transaction completed
pub type SETUP_R = crate::BitReader;
///Field `STATRX` writer - Status bits, for reception transfers
pub type STATRX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTOGRX` writer - Data Toggle, for reception transfers
pub type DTOGRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VTRX` reader - USB valid transaction received
pub type VTRX_R = crate::BitReader;
///Field `VTRX` writer - USB valid transaction received
pub type VTRX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEVADDR` reader - Host mode
pub type DEVADDR_R = crate::FieldReader;
///Field `DEVADDR` writer - Host mode
pub type DEVADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NAK` reader - Host mode
pub type NAK_R = crate::BitReader;
///Field `NAK` writer - Host mode
pub type NAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LS_EP` reader - Low speed endpoint host with HUB only
pub type LS_EP_R = crate::BitReader;
///Field `LS_EP` writer - Low speed endpoint host with HUB only
pub type LS_EP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_TX` reader - Received error for an OUT/SETUP transaction
pub type ERR_TX_R = crate::BitReader;
///Field `ERR_TX` writer - Received error for an OUT/SETUP transaction
pub type ERR_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_RX` reader - Received error for an IN transaction
pub type ERR_RX_R = crate::BitReader;
///Field `ERR_RX` writer - Received error for an IN transaction
pub type ERR_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THREE_ERR_TX` reader - Three errors for an OUT or SETUP transaction
pub type THREE_ERR_TX_R = crate::FieldReader;
///Field `THREE_ERR_TX` writer - Three errors for an OUT or SETUP transaction
pub type THREE_ERR_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `THREE_ERR_RX` reader - Three errors for an IN transaction
pub type THREE_ERR_RX_R = crate::FieldReader;
///Field `THREE_ERR_RX` writer - Three errors for an IN transaction
pub type THREE_ERR_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - endpoint/channel address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 7 - Valid USB transaction transmitted
    #[inline(always)]
    pub fn vttx(&self) -> VTTX_R {
        VTTX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - endpoint/channel kind
    #[inline(always)]
    pub fn epkind(&self) -> EPKIND_R {
        EPKIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - USB type of transaction
    #[inline(always)]
    pub fn utype(&self) -> UTYPE_R {
        UTYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - USB valid transaction received
    #[inline(always)]
    pub fn vtrx(&self) -> VTRX_R {
        VTRX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:22 - Host mode
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 23 - Host mode
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Low speed endpoint host with HUB only
    #[inline(always)]
    pub fn ls_ep(&self) -> LS_EP_R {
        LS_EP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Received error for an OUT/SETUP transaction
    #[inline(always)]
    pub fn err_tx(&self) -> ERR_TX_R {
        ERR_TX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Received error for an IN transaction
    #[inline(always)]
    pub fn err_rx(&self) -> ERR_RX_R {
        ERR_RX_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - Three errors for an OUT or SETUP transaction
    #[inline(always)]
    pub fn three_err_tx(&self) -> THREE_ERR_TX_R {
        THREE_ERR_TX_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bits 29:30 - Three errors for an IN transaction
    #[inline(always)]
    pub fn three_err_rx(&self) -> THREE_ERR_RX_R {
        THREE_ERR_RX_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHEPR")
            .field("ea", &self.ea())
            .field("vttx", &self.vttx())
            .field("epkind", &self.epkind())
            .field("utype", &self.utype())
            .field("setup", &self.setup())
            .field("vtrx", &self.vtrx())
            .field("devaddr", &self.devaddr())
            .field("nak", &self.nak())
            .field("ls_ep", &self.ls_ep())
            .field("err_tx", &self.err_tx())
            .field("err_rx", &self.err_rx())
            .field("three_err_tx", &self.three_err_tx())
            .field("three_err_rx", &self.three_err_rx())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - endpoint/channel address
    #[inline(always)]
    pub fn ea(&mut self) -> EA_W<CHEPRrs> {
        EA_W::new(self, 0)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stattx(&mut self) -> STATTX_W<CHEPRrs> {
        STATTX_W::new(self, 4)
    }
    ///Bit 6 - Data toggle, for transmission transfers
    #[inline(always)]
    pub fn dtogtx(&mut self) -> DTOGTX_W<CHEPRrs> {
        DTOGTX_W::new(self, 6)
    }
    ///Bit 7 - Valid USB transaction transmitted
    #[inline(always)]
    pub fn vttx(&mut self) -> VTTX_W<CHEPRrs> {
        VTTX_W::new(self, 7)
    }
    ///Bit 8 - endpoint/channel kind
    #[inline(always)]
    pub fn epkind(&mut self) -> EPKIND_W<CHEPRrs> {
        EPKIND_W::new(self, 8)
    }
    ///Bits 9:10 - USB type of transaction
    #[inline(always)]
    pub fn utype(&mut self) -> UTYPE_W<CHEPRrs> {
        UTYPE_W::new(self, 9)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn statrx(&mut self) -> STATRX_W<CHEPRrs> {
        STATRX_W::new(self, 12)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtogrx(&mut self) -> DTOGRX_W<CHEPRrs> {
        DTOGRX_W::new(self, 14)
    }
    ///Bit 15 - USB valid transaction received
    #[inline(always)]
    pub fn vtrx(&mut self) -> VTRX_W<CHEPRrs> {
        VTRX_W::new(self, 15)
    }
    ///Bits 16:22 - Host mode
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W<CHEPRrs> {
        DEVADDR_W::new(self, 16)
    }
    ///Bit 23 - Host mode
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<CHEPRrs> {
        NAK_W::new(self, 23)
    }
    ///Bit 24 - Low speed endpoint host with HUB only
    #[inline(always)]
    pub fn ls_ep(&mut self) -> LS_EP_W<CHEPRrs> {
        LS_EP_W::new(self, 24)
    }
    ///Bit 25 - Received error for an OUT/SETUP transaction
    #[inline(always)]
    pub fn err_tx(&mut self) -> ERR_TX_W<CHEPRrs> {
        ERR_TX_W::new(self, 25)
    }
    ///Bit 26 - Received error for an IN transaction
    #[inline(always)]
    pub fn err_rx(&mut self) -> ERR_RX_W<CHEPRrs> {
        ERR_RX_W::new(self, 26)
    }
    ///Bits 27:28 - Three errors for an OUT or SETUP transaction
    #[inline(always)]
    pub fn three_err_tx(&mut self) -> THREE_ERR_TX_W<CHEPRrs> {
        THREE_ERR_TX_W::new(self, 27)
    }
    ///Bits 29:30 - Three errors for an IN transaction
    #[inline(always)]
    pub fn three_err_rx(&mut self) -> THREE_ERR_RX_W<CHEPRrs> {
        THREE_ERR_RX_W::new(self, 29)
    }
}
/**USB endpoint/channel %s register

You can [`read`](crate::Reg::read) this register and get [`chepr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chepr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#USB:CHEP[0]R)*/
pub struct CHEPRrs;
impl crate::RegisterSpec for CHEPRrs {
    type Ux = u32;
}
///`read()` method returns [`chepr::R`](R) reader structure
impl crate::Readable for CHEPRrs {}
///`write(|w| ..)` method takes [`chepr::W`](W) writer structure
impl crate::Writable for CHEPRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CHEP%sR to value 0
impl crate::Resettable for CHEPRrs {
    const RESET_VALUE: u32 = 0;
}
