#[doc = "Register `DADDR` reader"]
pub type R = crate::R<DADDRrs>;
#[doc = "Register `DADDR` writer"]
pub type W = crate::W<DADDRrs>;
#[doc = "Field `ADD` reader - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
pub type ADD_R = crate::FieldReader;
#[doc = "Field `ADD` writer - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EF` reader - Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
pub type EF_R = crate::BitReader;
#[doc = "Field `EF` writer - Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
pub type EF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel Address (EA) field in the associated USB_EPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<DADDRrs> {
        ADD_W::new(self, 0)
    }
    #[doc = "Bit 7 - Enable function This bit is set by the software to enable the USB device. The address of this device is contained in the following ADD\\[6:0\\]
bits. If this bit is at '0 no transactions are handled, irrespective of the settings of USB_EPnR registers."]
    #[inline(always)]
    #[must_use]
    pub fn ef(&mut self) -> EF_W<DADDRrs> {
        EF_W::new(self, 7)
    }
}
#[doc = "USB device address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DADDRrs;
impl crate::RegisterSpec for DADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daddr::R`](R) reader structure"]
impl crate::Readable for DADDRrs {}
#[doc = "`write(|w| ..)` method takes [`daddr::W`](W) writer structure"]
impl crate::Writable for DADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DADDR to value 0"]
impl crate::Resettable for DADDRrs {
    const RESET_VALUE: u32 = 0;
}
