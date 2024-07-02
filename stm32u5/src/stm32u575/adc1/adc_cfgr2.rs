///Register `ADC_CFGR2` reader
pub type R = crate::R<ADC_CFGR2rs>;
///Register `ADC_CFGR2` writer
pub type W = crate::W<ADC_CFGR2rs>;
///Field `ROVSE` reader - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ROVSE_R = crate::BitReader;
///Field `ROVSE` writer - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type ROVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JOVSE` reader - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type JOVSE_R = crate::BitReader;
///Field `JOVSE` writer - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
pub type JOVSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVSS` reader - Oversampling right shift This bit field is set and cleared by software to define the right shifting applied to the raw oversampling result. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSS_R = crate::FieldReader;
///Field `OVSS` writer - Oversampling right shift This bit field is set and cleared by software to define the right shifting applied to the raw oversampling result. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OVSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TROVS` reader - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TROVS_R = crate::BitReader;
///Field `TROVS` writer - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type TROVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROVSM` reader - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type ROVSM_R = crate::BitReader;
///Field `ROVSM` writer - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type ROVSM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BULB` reader - Bulb sampling mode This bit is set and cleared by software to select the bulb sampling mode. SMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type BULB_R = crate::BitReader;
///Field `BULB` writer - Bulb sampling mode This bit is set and cleared by software to select the bulb sampling mode. SMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type BULB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SWTRIG_R = crate::BitReader;
///Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SWTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `SMPTRIG` reader - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN\[1:0\]
bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN\[1:0\]
bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).*/
pub type SMPTRIG_R = crate::BitReader;
/**Field `SMPTRIG` writer - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN\[1:0\]
bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN\[1:0\]
bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).*/
pub type SMPTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OSR` reader - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. 2: 3x ... 1023: 1024x Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OSR_R = crate::FieldReader<u16>;
///Field `OSR` writer - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. 2: 3x ... 1023: 1024x Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type OSR_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `LFTRIG` reader - Low-frequency trigger This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type LFTRIG_R = crate::BitReader;
///Field `LFTRIG` writer - Low-frequency trigger This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type LFTRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSHIFT` reader - Left shift factor This bitfield is set and cleared by software to define the left shifting applied to the final result with or without oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type LSHIFT_R = crate::FieldReader;
///Field `LSHIFT` writer - Left shift factor This bitfield is set and cleared by software to define the left shifting applied to the final result with or without oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type LSHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 5:8 - Oversampling right shift This bit field is set and cleared by software to define the right shifting applied to the raw oversampling result. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Bulb sampling mode This bit is set and cleared by software to select the bulb sampling mode. SMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 14) & 1) != 0)
    }
    /**Bit 15 - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN\[1:0\]
    bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN\[1:0\]
    bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25 - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. 2: 3x ... 1023: 1024x Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 27 - Low-frequency trigger This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lftrig(&self) -> LFTRIG_R {
        LFTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - Left shift factor This bitfield is set and cleared by software to define the left shifting applied to the final result with or without oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CFGR2")
            .field("rovse", &self.rovse())
            .field("jovse", &self.jovse())
            .field("ovss", &self.ovss())
            .field("trovs", &self.trovs())
            .field("rovsm", &self.rovsm())
            .field("bulb", &self.bulb())
            .field("swtrig", &self.swtrig())
            .field("smptrig", &self.smptrig())
            .field("osr", &self.osr())
            .field("lftrig", &self.lftrig())
            .field("lshift", &self.lshift())
            .finish()
    }
}
impl W {
    ///Bit 0 - Regular Oversampling Enable This bit is set and cleared by software to enable regular oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<ADC_CFGR2rs> {
        ROVSE_W::new(self, 0)
    }
    ///Bit 1 - Injected Oversampling Enable This bit is set and cleared by software to enable injected oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which ensures that no conversion is ongoing)
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<ADC_CFGR2rs> {
        JOVSE_W::new(self, 1)
    }
    ///Bits 5:8 - Oversampling right shift This bit field is set and cleared by software to define the right shifting applied to the raw oversampling result. Others: Reserved, must not be used. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<ADC_CFGR2rs> {
        OVSS_W::new(self, 5)
    }
    ///Bit 9 - Triggered Regular Oversampling This bit is set and cleared by software to enable triggered oversampling Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<ADC_CFGR2rs> {
        TROVS_W::new(self, 9)
    }
    ///Bit 10 - Regular Oversampling mode This bit is set and cleared by software to select the regular oversampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<ADC_CFGR2rs> {
        ROVSM_W::new(self, 10)
    }
    ///Bit 13 - Bulb sampling mode This bit is set and cleared by software to select the bulb sampling mode. SMPTRIG bit must not be set when the BULB bit is set. The very first ADC conversion is performed with the sampling time specified in SMPx bits. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn bulb(&mut self) -> BULB_W<ADC_CFGR2rs> {
        BULB_W::new(self, 13)
    }
    ///Bit 14 - Software trigger bit for sampling time control trigger mode This bit is set and cleared by software to enable the bulb sampling mode. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<ADC_CFGR2rs> {
        SWTRIG_W::new(self, 14)
    }
    /**Bit 15 - Sampling time control trigger mode This bit is set and cleared by software to enable the sampling time control trigger mode. The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge. EXTEN\[1:0\]
    bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set. When EXTEN\[1:0\]
    bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start the conversion. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    #[must_use]
    pub fn smptrig(&mut self) -> SMPTRIG_W<ADC_CFGR2rs> {
        SMPTRIG_W::new(self, 15)
    }
    ///Bits 16:25 - Oversampling ratio This bitfield is set and cleared by software to define the oversampling ratio. 2: 3x ... 1023: 1024x Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<ADC_CFGR2rs> {
        OSR_W::new(self, 16)
    }
    ///Bit 27 - Low-frequency trigger This bit is set and cleared by software Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn lftrig(&mut self) -> LFTRIG_W<ADC_CFGR2rs> {
        LFTRIG_W::new(self, 27)
    }
    ///Bits 28:31 - Left shift factor This bitfield is set and cleared by software to define the left shifting applied to the final result with or without oversampling. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LSHIFT_W<ADC_CFGR2rs> {
        LSHIFT_W::new(self, 28)
    }
}
/**ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`adc_cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#ADC1:ADC_CFGR2)*/
pub struct ADC_CFGR2rs;
impl crate::RegisterSpec for ADC_CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`adc_cfgr2::R`](R) reader structure
impl crate::Readable for ADC_CFGR2rs {}
///`write(|w| ..)` method takes [`adc_cfgr2::W`](W) writer structure
impl crate::Writable for ADC_CFGR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ADC_CFGR2 to value 0
impl crate::Resettable for ADC_CFGR2rs {
    const RESET_VALUE: u32 = 0;
}
