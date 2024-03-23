#[doc = "Register `DEVR0` reader"]
pub type R = crate::R<DEVR0rs>;
#[doc = "Register `DEVR0` writer"]
pub type W = crate::W<DEVR0rs>;
#[doc = "Field `DAVAL` reader - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_R = crate::BitReader;
#[doc = "Field `DAVAL` writer - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
pub type DAVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_R = crate::FieldReader;
#[doc = "Field `DA` writer - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
pub type DA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IBIEN` reader - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_R = crate::BitReader;
#[doc = "Field `IBIEN` writer - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
pub type IBIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CREN` reader - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_R = crate::BitReader;
#[doc = "Field `CREN` writer - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
pub type CREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HJEN` reader - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_R = crate::BitReader;
#[doc = "Field `HJEN` writer - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
pub type HJEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AS` reader - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
pub type AS_R = crate::FieldReader;
#[doc = "Field `RSTACT` reader - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
pub type RSTACT_R = crate::FieldReader;
#[doc = "Field `RSTVAL` reader - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
pub type RSTVAL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    pub fn daval(&self) -> DAVAL_R {
        DAVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    pub fn ibien(&self) -> IBIEN_R {
        IBIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    pub fn cren(&self) -> CREN_R {
        CREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    pub fn hjen(&self) -> HJEN_R {
        HJEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - activity state (when the I3C is acting as target) This read field is updated by hardware on the reception of a ENTASx CCC (enter activity state, with x=0-3):"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - reset action/level on received reset pattern (when the I3C is acting as target) This read field is used by hardware on the reception of a direct read RSTACT CCC in order to return the corresponding data byte on the I3C bus. This read field is updated by hardware on the reception of a broadcast or direct write RSTACT CCC (target reset action). Only the defining bytes 0x00, 0x01 and 0x02 are mapped, and RSTACT\\[1:0\\]
= Defining Byte\\[1:0\\]. a) partially reset the I3C peripheral, by a write and clear of the enable bit of the i3C configuration register (i.e. write I3C_CFGR.EN=0). This reset the I3C bus interface and the I3C kernel sub-parts, without modifying the content of the I3C APB registers (excepted the I3C_CFGR.EN bit). b) reset fully the I3C peripheral including all its registers via a write and set to the I3C reset control bit of the RCC (Reset and Clock Controller) register. a system reset. This has the same impact as a pin reset (i.e. NRST=0) (refer to RCC functional description - Reset part): – the software writes and set the AICR.SYSRESETREQ register control bit, when the device is controlled by a CortexTM-M. – the software writes and set the RCC_GRSTCSETR.SYSRST=1, when the device is controlled by a CortexTM-A."]
    #[inline(always)]
    pub fn rstact(&self) -> RSTACT_R {
        RSTACT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - reset action is valid (when the I3C is acting as target) This read bit is asserted by hardware to indicate that the RTSACT\\[1:0\\]
field has been updated on the reception of a broadcast or direct write RSTACT CCC (target reset action) and is valid. This field is cleared by hardware when the target receives a frame start. If RSTVAL=1: when the RSTF is asserted (and/or the corresponding interrupt if enabled), I3C_DEVR0.RSTACT\\[1:0\\]
dictates the reset action to be performed by the software if any. If RSTVAL=0: when the RSTF is asserted (and/or the corresponding interrupt if enabled), the software should issue an I3C reset after a first detected reset pattern, and a system reset on the second one."]
    #[inline(always)]
    pub fn rstval(&self) -> RSTVAL_R {
        RSTVAL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dynamic address is valid (when the I3C is acting as target) When the I3C is acting as controller, this field can be written by software, for validating its own dynamic address, for example before a controller-role hand-off. When the I3C is acting as target, this field is asserted by hardware on the acknowledge of the broadcast ENTDAA CCC or the direct SETNEWDA CCC, and this field is cleared by hardware on the acknowledge of the broadcast RSTDAA CCC."]
    #[inline(always)]
    #[must_use]
    pub fn daval(&mut self) -> DAVAL_W<DEVR0rs> {
        DAVAL_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7-bit dynamic address When the I3C is acting as controller, this field can be written by software, for defining its own dynamic address. When the I3C is acting as target, this field is updated by hardware on the reception of either the broadcast ENTDAA CCC or the direct SETNEWDA CCC."]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<DEVR0rs> {
        DA_W::new(self, 1)
    }
    #[doc = "Bit 16 - IBI request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISINT=1 (i.e. cleared) and the reception of ENEC CCC with ENINT=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn ibien(&mut self) -> IBIEN_W<DEVR0rs> {
        IBIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - controller-role request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISCR=1 (i.e. cleared) and the reception of ENEC CCC with ENCR=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn cren(&mut self) -> CREN_W<DEVR0rs> {
        CREN_W::new(self, 17)
    }
    #[doc = "Bit 19 - hot-join request enable (when the I3C is acting as target) This field is initially written by software when I3C_CFGR.EN=0, and is updated by hardware on the reception of DISEC CCC with DISHJ=1 (i.e. cleared) and the reception of ENEC CCC with ENHJ=1 (i.e. set)."]
    #[inline(always)]
    #[must_use]
    pub fn hjen(&mut self) -> HJEN_W<DEVR0rs> {
        HJEN_W::new(self, 19)
    }
}
#[doc = "I3C own device characteristics register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVR0rs;
impl crate::RegisterSpec for DEVR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devr0::R`](R) reader structure"]
impl crate::Readable for DEVR0rs {}
#[doc = "`write(|w| ..)` method takes [`devr0::W`](W) writer structure"]
impl crate::Writable for DEVR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEVR0 to value 0"]
impl crate::Resettable for DEVR0rs {
    const RESET_VALUE: u32 = 0;
}
