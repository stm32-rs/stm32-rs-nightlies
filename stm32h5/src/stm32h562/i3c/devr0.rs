///Register `DEVR0` reader
pub type R = crate::R<DEVR0rs>;
///Register `DEVR0` writer
pub type W = crate::W<DEVR0rs>;
///Field `DAVAL` reader - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC.
pub type DAVAL_R = crate::BitReader;
///Field `DAVAL` writer - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC.
pub type DAVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DA` reader - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC.
pub type DA_R = crate::FieldReader;
///Field `DA` writer - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC.
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `IBIEN` reader - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).
pub type IBIEN_R = crate::BitReader;
///Field `IBIEN` writer - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).
pub type IBIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CREN` reader - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).
pub type CREN_R = crate::BitReader;
///Field `CREN` writer - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).
pub type CREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HJEN` reader - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).
pub type HJEN_R = crate::BitReader;
///Field `HJEN` writer - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).
pub type HJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AS` reader - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):
pub type AS_R = crate::FieldReader;
///Field `RSTACT` reader - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus.
pub type RSTACT_R = crate::FieldReader;
///Field `RSTVAL` reader - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\[1:0\] field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one.
pub type RSTVAL_R = crate::BitReader;
impl R {
    ///Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC.
    #[inline(always)]
    pub fn daval(&self) -> DAVAL_R {
        DAVAL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC.
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).
    #[inline(always)]
    pub fn ibien(&self) -> IBIEN_R {
        IBIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).
    #[inline(always)]
    pub fn cren(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).
    #[inline(always)]
    pub fn hjen(&self) -> HJEN_R {
        HJEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus.
    #[inline(always)]
    pub fn rstact(&self) -> RSTACT_R {
        RSTACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\[1:0\] field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\[1:0\] dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one.
    #[inline(always)]
    pub fn rstval(&self) -> RSTVAL_R {
        RSTVAL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVR0")
            .field("daval", &self.daval())
            .field("da", &self.da())
            .field("ibien", &self.ibien())
            .field("cren", &self.cren())
            .field("hjen", &self.hjen())
            .field("as_", &self.as_())
            .field("rstact", &self.rstact())
            .field("rstval", &self.rstval())
            .finish()
    }
}
impl W {
    ///Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC.
    #[inline(always)]
    pub fn daval(&mut self) -> DAVAL_W<'_, DEVR0rs> {
        DAVAL_W::new(self, 0)
    }
    ///Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC.
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DEVR0rs> {
        DA_W::new(self, 1)
    }
    ///Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set).
    #[inline(always)]
    pub fn ibien(&mut self) -> IBIEN_W<'_, DEVR0rs> {
        IBIEN_W::new(self, 16)
    }
    ///Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set).
    #[inline(always)]
    pub fn cren(&mut self) -> CREN_W<'_, DEVR0rs> {
        CREN_W::new(self, 17)
    }
    ///Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set).
    #[inline(always)]
    pub fn hjen(&mut self) -> HJEN_W<'_, DEVR0rs> {
        HJEN_W::new(self, 19)
    }
}
/**I3C own device characteristics register

You can [`read`](crate::Reg::read) this register and get [`devr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#I3C:DEVR0)*/
pub struct DEVR0rs;
impl crate::RegisterSpec for DEVR0rs {
    type Ux = u32;
}
///`read()` method returns [`devr0::R`](R) reader structure
impl crate::Readable for DEVR0rs {}
///`write(|w| ..)` method takes [`devr0::W`](W) writer structure
impl crate::Writable for DEVR0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DEVR0 to value 0
impl crate::Resettable for DEVR0rs {}
