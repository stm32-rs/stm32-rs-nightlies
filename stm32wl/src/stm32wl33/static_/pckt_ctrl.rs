///Register `PCKT_CTRL` reader
pub type R = crate::R<PCKT_CTRLrs>;
///Register `PCKT_CTRL` writer
pub type W = crate::W<PCKT_CTRLrs>;
///Field `PCKT_FORMAT` reader - Packet format
pub type PCKT_FORMAT_R = crate::BitReader;
///Field `PCKT_FORMAT` writer - Packet format
pub type PCKT_FORMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_SWAP` reader - Invert MSB-LSB transmission order (bitendianess)
pub type BYTE_SWAP_R = crate::BitReader;
///Field `BYTE_SWAP` writer - Invert MSB-LSB transmission order (bitendianess)
pub type BYTE_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOUR_FSK_SYM_SWAP` reader - Invert bit to symbol mapping for 4-(G)FSK
pub type FOUR_FSK_SYM_SWAP_R = crate::BitReader;
///Field `FOUR_FSK_SYM_SWAP` writer - Invert bit to symbol mapping for 4-(G)FSK
pub type FOUR_FSK_SYM_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_MODE` reader - RX mode
pub type RX_MODE_R = crate::FieldReader;
///Field `RX_MODE` writer - RX mode
pub type RX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TX_MODE` reader - TX mode
pub type TX_MODE_R = crate::FieldReader;
///Field `TX_MODE` writer - TX mode
pub type TX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WHIT_BF_FEC` reader - Whitening before FEC feature
pub type WHIT_BF_FEC_R = crate::BitReader;
///Field `WHIT_BF_FEC` writer - Whitening before FEC feature
pub type WHIT_BF_FEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHIT_EN` reader - Whitening enable
pub type WHIT_EN_R = crate::BitReader;
///Field `WHIT_EN` writer - Whitening enable
pub type WHIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHIT_INIT` reader - Whitening initialization value.
pub type WHIT_INIT_R = crate::FieldReader<u16>;
///Field `WHIT_INIT` writer - Whitening initialization value.
pub type WHIT_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `CODING_SEL` reader - Coding / decoding selection
pub type CODING_SEL_R = crate::FieldReader;
///Field `CODING_SEL` writer - Coding / decoding selection
pub type CODING_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MANCHESTER_TYPE` reader - Select the Manchester encoding polarity
pub type MANCHESTER_TYPE_R = crate::BitReader;
///Field `MANCHESTER_TYPE` writer - Select the Manchester encoding polarity
pub type MANCHESTER_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_EN_4G` reader - This field is used as Interleaving enable for 802.
pub type INT_EN_4G_R = crate::BitReader;
///Field `INT_EN_4G` writer - This field is used as Interleaving enable for 802.
pub type INT_EN_4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FEC_TYPE_4G` reader - FEC type for 802.
pub type FEC_TYPE_4G_R = crate::BitReader;
///Field `FEC_TYPE_4G` writer - FEC type for 802.
pub type FEC_TYPE_4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FCS_TYPE_4G` reader - FCS type value in header field for 802.
pub type FCS_TYPE_4G_R = crate::BitReader;
///Field `FCS_TYPE_4G` writer - FCS type value in header field for 802.
pub type FCS_TYPE_4G_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MOD_INTERP_EN` reader - Enable frequency interpolator (for 2-GFSK and 4-GFSK)
pub type MOD_INTERP_EN_R = crate::BitReader;
///Field `MOD_INTERP_EN` writer - Enable frequency interpolator (for 2-GFSK and 4-GFSK)
pub type MOD_INTERP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PN_SEL` reader - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')
pub type PN_SEL_R = crate::BitReader;
///Field `PN_SEL` writer - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')
pub type PN_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCE_2FSK_SYNC_MODE` reader - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK
pub type FORCE_2FSK_SYNC_MODE_R = crate::BitReader;
///Field `FORCE_2FSK_SYNC_MODE` writer - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK
pub type FORCE_2FSK_SYNC_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Packet format
    #[inline(always)]
    pub fn pckt_format(&self) -> PCKT_FORMAT_R {
        PCKT_FORMAT_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Invert MSB-LSB transmission order (bitendianess)
    #[inline(always)]
    pub fn byte_swap(&self) -> BYTE_SWAP_R {
        BYTE_SWAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Invert bit to symbol mapping for 4-(G)FSK
    #[inline(always)]
    pub fn four_fsk_sym_swap(&self) -> FOUR_FSK_SYM_SWAP_R {
        FOUR_FSK_SYM_SWAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - RX mode
    #[inline(always)]
    pub fn rx_mode(&self) -> RX_MODE_R {
        RX_MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:8 - TX mode
    #[inline(always)]
    pub fn tx_mode(&self) -> TX_MODE_R {
        TX_MODE_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 10 - Whitening before FEC feature
    #[inline(always)]
    pub fn whit_bf_fec(&self) -> WHIT_BF_FEC_R {
        WHIT_BF_FEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Whitening enable
    #[inline(always)]
    pub fn whit_en(&self) -> WHIT_EN_R {
        WHIT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:20 - Whitening initialization value.
    #[inline(always)]
    pub fn whit_init(&self) -> WHIT_INIT_R {
        WHIT_INIT_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    ///Bits 21:22 - Coding / decoding selection
    #[inline(always)]
    pub fn coding_sel(&self) -> CODING_SEL_R {
        CODING_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - Select the Manchester encoding polarity
    #[inline(always)]
    pub fn manchester_type(&self) -> MANCHESTER_TYPE_R {
        MANCHESTER_TYPE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - This field is used as Interleaving enable for 802.
    #[inline(always)]
    pub fn int_en_4g(&self) -> INT_EN_4G_R {
        INT_EN_4G_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - FEC type for 802.
    #[inline(always)]
    pub fn fec_type_4g(&self) -> FEC_TYPE_4G_R {
        FEC_TYPE_4G_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - FCS type value in header field for 802.
    #[inline(always)]
    pub fn fcs_type_4g(&self) -> FCS_TYPE_4G_R {
        FCS_TYPE_4G_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Enable frequency interpolator (for 2-GFSK and 4-GFSK)
    #[inline(always)]
    pub fn mod_interp_en(&self) -> MOD_INTERP_EN_R {
        MOD_INTERP_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')
    #[inline(always)]
    pub fn pn_sel(&self) -> PN_SEL_R {
        PN_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK
    #[inline(always)]
    pub fn force_2fsk_sync_mode(&self) -> FORCE_2FSK_SYNC_MODE_R {
        FORCE_2FSK_SYNC_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCKT_CTRL")
            .field("pckt_format", &self.pckt_format())
            .field("byte_swap", &self.byte_swap())
            .field("four_fsk_sym_swap", &self.four_fsk_sym_swap())
            .field("rx_mode", &self.rx_mode())
            .field("tx_mode", &self.tx_mode())
            .field("whit_bf_fec", &self.whit_bf_fec())
            .field("whit_en", &self.whit_en())
            .field("whit_init", &self.whit_init())
            .field("coding_sel", &self.coding_sel())
            .field("manchester_type", &self.manchester_type())
            .field("int_en_4g", &self.int_en_4g())
            .field("fec_type_4g", &self.fec_type_4g())
            .field("fcs_type_4g", &self.fcs_type_4g())
            .field("mod_interp_en", &self.mod_interp_en())
            .field("pn_sel", &self.pn_sel())
            .field("force_2fsk_sync_mode", &self.force_2fsk_sync_mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Packet format
    #[inline(always)]
    pub fn pckt_format(&mut self) -> PCKT_FORMAT_W<'_, PCKT_CTRLrs> {
        PCKT_FORMAT_W::new(self, 0)
    }
    ///Bit 2 - Invert MSB-LSB transmission order (bitendianess)
    #[inline(always)]
    pub fn byte_swap(&mut self) -> BYTE_SWAP_W<'_, PCKT_CTRLrs> {
        BYTE_SWAP_W::new(self, 2)
    }
    ///Bit 3 - Invert bit to symbol mapping for 4-(G)FSK
    #[inline(always)]
    pub fn four_fsk_sym_swap(&mut self) -> FOUR_FSK_SYM_SWAP_W<'_, PCKT_CTRLrs> {
        FOUR_FSK_SYM_SWAP_W::new(self, 3)
    }
    ///Bits 4:6 - RX mode
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RX_MODE_W<'_, PCKT_CTRLrs> {
        RX_MODE_W::new(self, 4)
    }
    ///Bits 7:8 - TX mode
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TX_MODE_W<'_, PCKT_CTRLrs> {
        TX_MODE_W::new(self, 7)
    }
    ///Bit 10 - Whitening before FEC feature
    #[inline(always)]
    pub fn whit_bf_fec(&mut self) -> WHIT_BF_FEC_W<'_, PCKT_CTRLrs> {
        WHIT_BF_FEC_W::new(self, 10)
    }
    ///Bit 11 - Whitening enable
    #[inline(always)]
    pub fn whit_en(&mut self) -> WHIT_EN_W<'_, PCKT_CTRLrs> {
        WHIT_EN_W::new(self, 11)
    }
    ///Bits 12:20 - Whitening initialization value.
    #[inline(always)]
    pub fn whit_init(&mut self) -> WHIT_INIT_W<'_, PCKT_CTRLrs> {
        WHIT_INIT_W::new(self, 12)
    }
    ///Bits 21:22 - Coding / decoding selection
    #[inline(always)]
    pub fn coding_sel(&mut self) -> CODING_SEL_W<'_, PCKT_CTRLrs> {
        CODING_SEL_W::new(self, 21)
    }
    ///Bit 24 - Select the Manchester encoding polarity
    #[inline(always)]
    pub fn manchester_type(&mut self) -> MANCHESTER_TYPE_W<'_, PCKT_CTRLrs> {
        MANCHESTER_TYPE_W::new(self, 24)
    }
    ///Bit 25 - This field is used as Interleaving enable for 802.
    #[inline(always)]
    pub fn int_en_4g(&mut self) -> INT_EN_4G_W<'_, PCKT_CTRLrs> {
        INT_EN_4G_W::new(self, 25)
    }
    ///Bit 26 - FEC type for 802.
    #[inline(always)]
    pub fn fec_type_4g(&mut self) -> FEC_TYPE_4G_W<'_, PCKT_CTRLrs> {
        FEC_TYPE_4G_W::new(self, 26)
    }
    ///Bit 27 - FCS type value in header field for 802.
    #[inline(always)]
    pub fn fcs_type_4g(&mut self) -> FCS_TYPE_4G_W<'_, PCKT_CTRLrs> {
        FCS_TYPE_4G_W::new(self, 27)
    }
    ///Bit 28 - Enable frequency interpolator (for 2-GFSK and 4-GFSK)
    #[inline(always)]
    pub fn mod_interp_en(&mut self) -> MOD_INTERP_EN_W<'_, PCKT_CTRLrs> {
        MOD_INTERP_EN_W::new(self, 28)
    }
    ///Bit 29 - Select the Pseudo Random Binary Sequence (PRBS) polynomial to apply when the selected transmission mode is PN mode (TX_MODE = '11')
    #[inline(always)]
    pub fn pn_sel(&mut self) -> PN_SEL_W<'_, PCKT_CTRLrs> {
        PN_SEL_W::new(self, 29)
    }
    ///Bit 31 - Force SYNC word to be formatted as a 2-(G)FSK bit steam instead of 4-(G)FSK
    #[inline(always)]
    pub fn force_2fsk_sync_mode(&mut self) -> FORCE_2FSK_SYNC_MODE_W<'_, PCKT_CTRLrs> {
        FORCE_2FSK_SYNC_MODE_W::new(self, 31)
    }
}
/**PCKT_CTRL register

You can [`read`](crate::Reg::read) this register and get [`pckt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pckt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:PCKT_CTRL)*/
pub struct PCKT_CTRLrs;
impl crate::RegisterSpec for PCKT_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`pckt_ctrl::R`](R) reader structure
impl crate::Readable for PCKT_CTRLrs {}
///`write(|w| ..)` method takes [`pckt_ctrl::W`](W) writer structure
impl crate::Writable for PCKT_CTRLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PCKT_CTRL to value 0
impl crate::Resettable for PCKT_CTRLrs {}
